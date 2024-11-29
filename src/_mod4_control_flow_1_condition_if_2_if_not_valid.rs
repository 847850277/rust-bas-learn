pub(crate) fn test() {
    let latitude = 1;
    //Warning: remove these parentheses
    if (latitude == 0) {
        let location = "Equator";
    }
    //Error: no block
    if latitude == 0 {
        let location = "Equator";
    }
}
