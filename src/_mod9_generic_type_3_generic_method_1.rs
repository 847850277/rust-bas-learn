fn swap<T>(s1: &mut T, s2: &mut T)
    where T: Copy {
    (*s1, *s2) = (*s2, *s1);
}

pub(crate) fn test() {

    let mut n1 = 5;
    let mut n2 = 7;
    swap(&mut n1, &mut n2);
    //n1 is 7 and n2 is 5
    let mut s1 = "cat";
    let mut s2 = "dog";
    swap(&mut s1, &mut s2);
    //s1[0] is "dog" and s2[0] is "cat"
    println!("n1 is {:?} and n2 is {:?}", n1, n2);
    println!("s1 is {:?} and s2 is {:?}", s1, s2);

}