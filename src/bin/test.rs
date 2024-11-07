use sorting_rs::{ bubble_sort, selection_sort, insertion_sort };

fn main() 
{
    let mut vec = vec![4, 7, 3, 6, 2, 7, 2, 8, 1, 9, 2];

    bubble_sort(&mut vec);
    println!("Bubble\t\t: {:?}", vec);

    vec = vec![4, 7, 3, 6, 2, 7, 2, 8, 1, 9, 2];
    selection_sort(&mut vec);
    println!("Selection\t: {:?}", vec);

    vec = vec![4, 7, 3, 6, 2, 7, 2, 8, 1, 9, 2];
    insertion_sort(&mut vec);
    println!("Insertion\t: {:?}", vec);
}
