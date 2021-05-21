use exercises::Exercise;

pub fn ex_1_1_1() -> Exercise {
    return Exercise {
        number: String::from("1.1-1"),
        question: String::from(
            "Give a real-world example that requires sorting or a real-world example \
            that requires computing a convex hull."
        ),
        answer: String::from(
            "A checkout system requires sorting if it has to implement some more advanced functionality \
            (e.g. some logic based on repeating items etc.)\n\
            but in general any system which does ranking or indexing requires sorting.\n\
            Convex hull could be used in image processing, e.g. you might want to do calculations \
            within an \"interesting area\" inside the convex hull.\n\
            References:\n\
            https://medium.com/@pascal.sommer.ch/a-gentle-introduction-to-the-convex-hull-problem-62dfcabee90c\n\
            https://luminousvillage.wordpress.com/2016/05/12/applications-of-the-convex-hull"
        ),
    };
}