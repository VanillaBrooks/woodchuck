use std::io::prelude::*;

pub struct LatexTable<'a, T>
where
    T: Read,
{
    pub reader: csv::Reader<T>,
    caption: &'a str,
    label: &'a str,
}
impl<'a, T> LatexTable<'a, T>
where
    T: Read,
{
    pub fn from_reader(reader: T, caption: &'a str, label: &'a str) -> Self {
        let reader = csv::Reader::from_reader(reader);
        let s = LatexTable {
            reader,
            caption,
            label,
        };
        s
    }

    fn column_spacings(&mut self) -> Result<String, csv::Error> {
        let record = self.reader.headers()?;
        let length = record.len();
        let mut out = String::with_capacity(1 + length * 2);

        out.push('|');
        for _ in 0..length {
            out.push_str("c|")
        }

        Ok(out)
    }

    fn column_headers(&mut self) -> Result<String, csv::Error> {
        let mut buffer = String::new();
        let record = self.reader.headers()?;
        row_sequence(&mut buffer, record);
        Ok(buffer)
    }

    pub fn make_header(&mut self) -> Result<String, csv::Error> {
        let table = "\\begin{table}[H]\n";
        let center = "\t\\begin{center}\n";
        let tabular = "\t\t\\begin{tabular} { ";
        let inner = self.column_spacings()?;
        let outer = " }\n";

        let mut out =
            String::with_capacity(center.len() + tabular.len() + inner.len() + outer.len());
        out.push_str(table);
        out.push_str(center);
        out.push_str(tabular);
        out.push_str(&inner);
        out.push_str(outer);

        Ok(out)
    }

    pub fn make_ender(&self) -> String {
        let table = "\\end{table}\n";
        let extras = self.table_extras();
        let tabular = "\t\t\\end{tabular}\n";
        let center = "\t\\end{center}\n";
        let mut out =
            String::with_capacity(extras.len() + table.len() + tabular.len() + center.len());

        out.push_str(tabular);
        out.push_str(&extras);
        out.push_str(center);
        out.push_str(table);
        out
    }

    pub fn make_table(&mut self) -> Result<String, csv::Error> {
        let mut out = String::new();

        for row in self.reader.records() {
            let row = row?;
            row_sequence(&mut out, &row);
        }

        Ok(out)
    }

    fn table_extras(&self) -> String {
        let mut base = String::with_capacity(20);
        base.push_str("\t\t\\caption{");
        base.push_str(self.caption);
        base.push_str("}\n\t\t\\label{");
        base.push_str(self.label);
        base.push_str("}\n");
        base
    }

    pub fn to_writer<W: Write>(&mut self, mut writer: W) -> Result<(), csv::Error> {
        writer.write_all(self.make_header()?.as_bytes())?;
        writer.write_all(self.column_headers()?.as_bytes())?;
        writer.write_all(self.make_table()?.as_bytes())?;
        writer.write_all(self.make_ender().as_bytes())?;
        Ok(())
    }
}

fn row_sequence(buffer: &mut String, row: &csv::StringRecord) {
    let mut first = true;
    for i in row.into_iter() {
        if first {
            buffer.push_str(&format!("\t\t\t{}", i));
            first = false;
        } else {
            buffer.push_str(&format!(" & {}", i))
        }
    }
    buffer.push_str("\\\\ \\hline \n");
}
