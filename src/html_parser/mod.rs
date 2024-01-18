
struct HtmlParser {
    html: String,
    tree: Ele
}

struct Ele { 
    tag: String,
    children: Vec<Box<Ele>>,
}







mod tests { 

}