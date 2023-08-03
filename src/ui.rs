slint::slint! {
    // component TestImage inherits Rectangle {
    //     Image {
    //         source: @image-url("icons/bus.png");
    //         width: parent.width;
    //         height: parent.height;
    //     }
    //  }

    export component MainWindow inherits Window {
        preferred-width: 600px;
        preferred-height: 400px;

        Text {
            text: "Sample Text";
            color: white;
        }

        // TestImage {}
   }
}
