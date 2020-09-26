use git_odb::alternate;
use std::{fs, io, path::PathBuf};

fn alternate(objects_at: impl Into<PathBuf>, objects_to: impl Into<PathBuf>) -> Result<(PathBuf, PathBuf), io::Error> {
    alternate_with(objects_at, objects_to, None)
}

fn alternate_with(
    objects_at: impl Into<PathBuf>,
    objects_to: impl Into<PathBuf>,
    content_before_to: Option<&str>,
) -> Result<(PathBuf, PathBuf), io::Error> {
    let at = objects_at.into();
    let to = objects_to.into();
    let at_info = at.join("info");
    fs::create_dir_all(&at_info)?;
    fs::create_dir_all(&to)?;
    let contents = if let Some(content) = content_before_to {
        let mut c = vec![b'\n'];
        c.extend(content.as_bytes());
        c.extend(to.to_string_lossy().as_bytes());
        c
    } else {
        to.to_string_lossy().as_bytes().to_owned()
    };
    fs::write(at_info.join("alternates"), contents)?;
    Ok((at, to))
}

#[test]
fn circular_alternates_are_detected() -> crate::Result {
    let tmp = tempdir::TempDir::new("alternates")?;
    let (from, _) = alternate(tmp.path().join("a"), tmp.path().join("b"))?;
    alternate(tmp.path().join("b"), tmp.path().join("a"))?;

    match alternate::resolve(&from) {
        Err(alternate::Error::Cycle(chain)) => {
            assert_eq!(
                chain
                    .into_iter()
                    .map(|p| p.file_name().expect("non-root").to_str().expect("utf8").to_owned())
                    .collect::<Vec<_>>(),
                vec!["a", "b"]
            );
        }
        _ => unreachable!("should be a specific kind of error"),
    }
    Ok(())
}

#[test]
fn single_link() -> crate::Result {
    let tmp = tempdir::TempDir::new("alternates")?;
    let non_alternate = tmp.path().join("actual");

    let (from, to) = alternate(tmp.path().join("a"), non_alternate)?;
    let alternates = alternate::resolve(from)?;
    assert_eq!(alternates.len(), 1);
    assert_eq!(alternates[0].loose.path, to);
    Ok(())
}

#[test]
fn no_alternate_in_first_objects_dir() -> crate::Result {
    let tmp = tempdir::TempDir::new("alternates")?;
    assert!(alternate::resolve(tmp.path())?.is_empty());
    Ok(())
}
