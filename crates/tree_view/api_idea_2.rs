fn main(){

    TreeView::dir(uuid).show(ui,|ui|{

        ui.leaf(id, |ui|{
            ui.label("leaf");
        });

        ui.dir(id,|ui|{
            ui.label("dir");
        }, |ui|{

            ui.leaf(leaf.id,|ui|{

            });
        })
    });


    let root = TreeView::dir(uuid);
    root.leaf(leaf.id, |ui|{
        ui.label("leaf");
    });
    root.leaf(leaf.id, |ui|{
        ui.label("leaf");
    });

    let dir = root.dir(dir.id, |ui|{
        ui.label("dir");
    });

    let dir = root.dir("Directory");
    dir.leaf("dir/leaf");
    dir.leaf("dir/leaf");
    dir.leaf("dir/leaf");

    let faz = dir.dir("Faz");
    faz.leaf("dir/faz/leaf");
    faz.leaf("dir/faz/leaf");

    dir.leaf("dir/other");

    root.leaf("whoo");


}