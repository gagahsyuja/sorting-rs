pub fn bubble(vec: &mut Vec<i32>)
{
    for i in 0..vec.len() - 1
    {
        for j in 0..vec.len() - i - 1
        {
            if vec[j] > vec[j + 1]
            {
                let temp = vec[j];
                vec[j] = vec[j + 1];
                vec[j + 1] = temp;
            }
        }
    }
}
