import os
import shutil

def copy_lib():

    old_path = "../target/debug/"
    new_path = "./"
    old_name = "libpy_nrf.dylib"
    new_name = "py_nrf.so"

    shutil.copy(old_path + old_name, new_path + new_name)


if __name__=="__main__":
    copy_lib()
    import py_nrf
    from model import Model

    myModel = Model()

    my_rust_model = py_nrf.UserModel(myModel)
    my_rust_model.set_iteratives_rust([2.0, 3.0])
    print(myModel.inputs)
    my_rust_model.evaluate_rust()
    print(my_rust_model.len_problem_rust())
    print(myModel.left)
    print(myModel.right)
