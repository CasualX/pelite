use super::{Directory, Entry, Resources};
use std::fmt;

/// Art used to format a directory tree.
#[derive(Debug)]
struct Art {
    margin_draw: &'static str,
    margin_open: &'static str,
    dir_entry: &'static str,
    dir_tail: &'static str,
    file_entry: &'static str,
    file_tail: &'static str,
}
/// Uses [box-drawing characters](https://en.wikipedia.org/wiki/Box-drawing_character) to draw the tree art.
static U: Art = Art {
    margin_draw: "│   ",
    margin_open: "    ",
    dir_entry: "├── ",
    dir_tail: "└── ",
    file_entry: "├── ",
    file_tail: "└── ",
};
/// Uses ascii to draw the tree art.
static A: Art = Art {
    margin_draw: "|   ",
    margin_open: "    ",
    dir_entry: "+-- ",
    dir_tail: "`-- ",
    file_entry: "+-- ",
    file_tail: "`-- ",
};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[allow(dead_code)]
enum TreeArt {
    Ascii,
    Unicode,
}

#[derive(Clone, Debug)]
struct TreeFmt<'a: 'd, 'd> {
    dir: &'d Directory<'a>,
    art: &'static Art,
    depth: u32,
    margin: u32,
}
impl<'a, 'd> TreeFmt<'a, 'd> {
    fn root(root: &'d Directory<'a>, art: TreeArt) -> TreeFmt<'a, 'd> {
        let art = match art {
            TreeArt::Ascii => &A,
            TreeArt::Unicode => &U,
        };
        TreeFmt {
            dir: root,
            art,
            depth: !0,
            margin: 0,
        }
    }
    fn dir(dir: &'d Directory<'a>, art: TreeArt) -> TreeFmt<'a, 'd> {
        let art = match art {
            TreeArt::Ascii => &A,
            TreeArt::Unicode => &U,
        };
        TreeFmt {
            dir,
            art,
            depth: 0,
            margin: 0,
        }
    }

    fn draw<F: fmt::Write>(&self, f: &mut F) -> fmt::Result {
        // Encode if root in depth
        let (root, depth) = if self.depth == !0 {
            (true, 0)
        } else {
            (false, self.depth)
        };

        // Quiet failsafe, unlikely to happen
        if depth >= 32 {
            return Ok(());
        }

        let mut entries = self.dir.entries();
        while let Some(e) = entries.next() {
            // Print the margin
            for open in (0..depth).map(|i| self.margin & (1 << i) != 0) {
                f.write_str(if open {
                    self.art.margin_open
                } else {
                    self.art.margin_draw
                })?;
            }
            // Write the prefix
            let tail = entries.len() == 0;
            let prefix = match (tail, e.is_dir()) {
                (false, false) => self.art.file_entry,
                (true, false) => self.art.file_tail,
                (false, true) => self.art.dir_entry,
                (true, true) => self.art.dir_tail,
            };
            f.write_str(prefix)?;
            // Print the file_name
            match e.name() {
                Ok(name) => write!(
                    f,
                    "{}",
                    name.rename_id(if root { &super::RSRC_TYPES } else { &[] })
                ),
                Err(err) => write!(f, "{}", err),
            }
            .and_then(|_| f.write_str(if e.is_dir() { "/\n" } else { "\n" }))?;
            // If it's a directory, print it recursively
            if let Ok(Entry::Directory(dir)) = e.entry() {
                TreeFmt {
                    dir: &dir,
                    art: self.art,
                    depth: depth + 1,
                    margin: self.margin | (tail as u32) << depth,
                }
                .draw(f)?;
            }
        }
        Ok(())
    }
}

impl<'a> fmt::Display for Resources<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Resources/\n")?;
        match self.root() {
            Ok(root) => TreeFmt::root(&root, TreeArt::Ascii).draw(f),
            Err(err) => err.fmt(f),
        }
    }
}

impl<'a> fmt::Display for Directory<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("Directory/\n")?;
        TreeFmt::dir(self, TreeArt::Ascii).draw(f)
    }
}
