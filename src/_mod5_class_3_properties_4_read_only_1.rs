struct FilmList {}
impl FilmList {
    fn count(self) -> i16 {
        return 10;
    }
}
pub(crate) fn test() {

    let film_list = FilmList{};
    let count = film_list.count();
    //count is 10
    println!("count is {}",count);

}
