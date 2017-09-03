extern crate quick_csv;
extern crate quick_xml;

use std::fs::File;
use std::vec::Vec;
use quick_xml::writer::Writer;
use quick_xml::events::{Event, BytesEnd, BytesStart};

fn main() {
    let csv = quick_csv::Csv::from_file("test.csv").expect("A csv file");
    let buffer = File::create("foo.xml").expect("no file created");
    let mut writer = Writer::new(buffer );
    let mut header =  Vec::<String>::new();

    let encoding = b"?xml version=\"1.0\" encoding=\"utf-8\"?";

    writer.write_event(Event::Start(BytesStart::owned(encoding.to_vec(),encoding.len())));
    writer.write("\n".as_bytes());
    let category = b"my_categoty";

    writer.write_event(Event::Start(BytesStart::owned(category.to_vec(),category.len())));
    writer.write("\n".as_bytes());
    for (i,row) in csv.into_iter().enumerate()  {
        if i == 0 {

            for col in row.expect("no row").columns().expect("cannot convert to utf8") {
                header.push(col.to_owned())
            }
        } else {
            let record = BytesStart::owned(b"Record".to_vec(),b"Record".len());
            writer.write("  ".as_bytes());
            writer.write_event(Event::Start(record));
            writer.write("\n".as_bytes());

            for (name, col) in header.iter().zip( row.expect("no row").columns().expect("cannot convert to utf8")) {

                let elem = BytesStart::borrowed(name.as_bytes(),name.len());

                writer.write("    ".as_bytes());
                writer.write_event(Event::Start(elem));
                writer.write(col.as_bytes());
                writer.write_event(Event::End(BytesEnd::borrowed(name.as_bytes())));
                writer.write("\n".as_bytes());

            }
            writer.write("  ".as_bytes());
            writer.write_event(Event::End(BytesEnd::borrowed(b"Record")));
            writer.write("\n".as_bytes());
        }
//        if i > 5 {
//            break;
//        }
    }
    writer.write_event(Event::End(BytesEnd::borrowed(category)));
    writer.write("\n".as_bytes());
}
