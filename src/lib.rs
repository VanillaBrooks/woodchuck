use std::io::prelude::*;

pub struct LatexTable<'a, T>
where
    T: Read,
{
    pub reader: csv::Reader<T>,
    caption: &'a str,
    label: &'a str,
    table_format: &'a str,
    table_args: &'a str,
    headers: Headers,
}
impl<'a, T> LatexTable<'a, T>
where
    T: Read,
{
    pub fn from_reader(
        reader: T,
        caption: &'a str,
        label: &'a str,
        table_format: &'a str,
        table_args: &'a str,
        headers: Headers,
    ) -> Self {
        let reader = csv::Reader::from_reader(reader);
        LatexTable {
            reader,
            caption,
            label,
            table_format,
            table_args,
            headers,
        }
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
        //let record = self.reader.headers()?;
        //row_sequence(&mut buffer, record);
        self.headers.write_header(&mut self.reader, &mut buffer)?;
        Ok(buffer)
    }

    pub fn make_header(&mut self) -> Result<String, csv::Error> {
        let table_header = format!(
            "
\\begin{{table{format}}}{arg}
    \\begin{{center}}
        \\begin{{tabular}} {{ {column_spacings} }}
        \\hline
",
            column_spacings = self.column_spacings()?,
            arg = self.table_args,
            format = self.table_format
        );

        Ok(table_header)
    }

    pub fn make_ender(&self) -> String {
        //let table = "\\end{table";
        //let table_end = format!("{}}}\n", self.table_format);
        //let extras = self.table_extras();
        //let tabular = "\t\t\\end{tabular}\n";
        //let center = "\t\\end{center}\n";

        format!(
            "       \\end{{tabular}}
    \\end{{center}}
    {extras}
\\end{{table{table_format}}}",
            table_format = self.table_format,
            extras = self.table_extras()
        )
    }

    pub fn make_table(&mut self) -> Result<String, csv::Error> {
        let mut out = String::new();

        for row in self.reader.records().skip(self.headers.row_skip()) {
            let row = row?;
            row_sequence(&mut out, &row);
        }

        Ok(out)
    }

    fn table_extras(&self) -> String {
        format!(
            "\\caption{{{cap}}}
    \\label{{{label}}}",
            cap = self.caption,
            label = self.label
        )
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

fn extended_headers<R: Read>(
    buffer: &mut String,
    reader: &mut csv::Reader<R>,
) -> Result<(), csv::Error> {
    let row_1 = reader.headers()?.clone();
    let row_2 = reader.records().next().unwrap().unwrap();

    let mut first = true;

    let mut line_start = 1;
    let mut line_start_stops = vec![];

    for i in 0..row_1.len() {
        // if this is a blank item then it will be handled elsewhere
        let text = if let Some(col_title) = row_1.get(i) {
            if col_title.is_empty()  {
                continue;
            } else {
                col_title
            }
        } else {
            continue;
        };

        // first, check how many of the rows next to us are empty
        let mut num_columns = 1;

        for j in i + 1..row_1.len() {
            // if the row is empty then make this column take more space
            if row_1.get(j).map(|x| x.is_empty()).unwrap_or(false) {
                num_columns += 1
            } else {
                break;
            }
        }

        let num_rows = if row_2.get(i).map(|x| x.is_empty() ).unwrap_or(false) && num_columns == 1 {
            2
        } else {
            1
        };

        assert!(
            !(num_rows > 1 && num_columns > 1),
            "multi row and multi column parsed"
        );

        let text = match (text, num_rows, num_columns) {
            (title, 1, 1) => title.to_string(),
            (title, num_rows, 1) if num_rows > 1 => {
                // multi-row case
                line_start_stops.push((line_start, i + 1));
                // i+1 gives the 1 based indexing, so i+2 gives 1 based indexing of the next
                // element
                line_start = i + 2;
                format!("\\multirow{{ {} }}{{*}}{{ {} }}", num_rows, title)
            }
            (title, 1, num_columns) if num_columns > 1 => {
                format!("\\multicolumn{{ {} }}{{ |c| }}{{ {} }}", num_columns, title)
            }
            _ => unreachable!(),
        };

        if first {
            buffer.push_str(&format!("\t\t\t{}", text));
            first = false;
        } else {
            buffer.push_str(&format!(" & {}", text));
        }
    }

    // required to start a new row
    buffer.push_str(r"\\");

    for (start_line, end_line) in line_start_stops {
        if start_line == end_line {
            continue;
        }
        buffer.push_str(&format!("\\cline{{ {} - {}  }}", start_line, end_line));
    }

    // handle the most recent start_line end_line
    if line_start < row_1.len() {
        buffer.push_str(&format!("\\cline{{ {} - {}  }}", line_start, row_1.len()));
    }

    row_sequence(buffer, &row_2);

    Ok(())
}

pub enum Headers {
    Simple,
    Extended,
}

impl Headers {
    fn write_header<R: Read>(
        &self,
        reader: &mut csv::Reader<R>,
        buffer: &mut String,
    ) -> Result<(), csv::Error> {
        match self {
            Headers::Simple => {
                let row = reader.headers()?;
                row_sequence(buffer, row);
            }
            Headers::Extended => {
                extended_headers(buffer, reader)?;
            }
        }

        Ok(())
    }

    fn row_skip(&self) -> usize {
        match self {
            Self::Simple => 0,
            Self::Extended => 1,
        }
    }

    pub fn from_cli_bool(multirow: bool) -> Self {
        if multirow {
            Self::Extended
        } else {
            Self::Simple
        }
    }
}
