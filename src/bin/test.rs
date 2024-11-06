use sorting_rs::bubble_sort;

fn main() 
{
    let mut vec = vec![4, 7, 3, 6, 2, 7, 2, 8, 1, 9, 2];

    // let sort = Bubble::new(vec);
    bubble_sort(&mut vec);

    println!("{:#?}", vec);
}
