#[derive(Debug)]
pub struct Vecna<'a, T: std::fmt::Display>(pub &'a Vec<T>);
impl<'a, T: std::fmt::Display> std::fmt::Display for Vecna<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[")?;
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(f, " {first}")?;
        };

        for x in iter {
            write!(f, ", {x}")?;
        }

        write!(f, " ])")
    }
}

impl<'a, T: std::fmt::Display> Vecna<'a, T> {
    pub fn pretty_print(&self, f: &mut std::fmt::Formatter<'_>, indent: &str) -> std::fmt::Result {
        write!(f, "[\n")?;
        let mut iter = self.0.iter();
        if let Some(first) = iter.next() {
            write!(f, "{indent}    {first}")?;
        };

        for x in iter {
            write!(f, ",\n{indent}    {x}")?;
        }

        write!(f, "\n{indent}]\n")
    }
}
