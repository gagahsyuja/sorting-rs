pub fn insertion_sort<T>(vec: &mut Vec<T>) where T: PartialOrd + Copy
{
    for i in 0..vec.len()
    {
        let mut position = i;

        while position > 0
        {
            if vec[position] < vec[position - 1]
            {
                let temp = vec[position];
                vec[position] = vec[position - 1];
                vec[position - 1] = temp;
            }

            position -= 1;
        }

    }
}
