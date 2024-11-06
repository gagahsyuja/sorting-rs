pub fn selection_sort<T>(vec: &mut Vec<T>) where T: PartialOrd + Copy
{
    for i in 0..vec.len()
    {
        for j in i + 1..vec.len()
        {
            if vec[i] > vec[j]
            {
                let temp = vec[i];
                vec[i] = vec[j];
                vec[j] = temp;
            }
        }
    }
}
