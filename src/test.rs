slint::slint!{
    import { Button, VerticalBox } from "std-widgets.slint";
    export component App inherits Window {
        property <int> counter: 1;

        VerticalBox {
            Text { text: "Hello -> " + counter; }
            Button { text: "yay"; clicked => {counter += 1;}}
        }
    }
}

fn main() {
    App::new().unwrap().run().unwrap();
    println!("Tester")
}