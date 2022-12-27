struct Person{
    name: String
}

struct House<'a>{
    people: Vec<& 'a mut Person>
    // The next line doesn't work:
    // people: Vec<& 'a Person>
    // If turns out we need to make the references
    // mutable in order to be able to mutate them
    // in a member function when self is mutable.
}

impl<'a> House<'a>
{
    fn update_name(&mut self, name: String){
        let maybe_p = self.people.last_mut();
        if let Some(p) = maybe_p{
            p.name = name;
        }
    }
}

fn main() {}
