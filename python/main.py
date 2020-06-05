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

    path = r'./data/config.xml'

    myModel = Model()
    myModel.set_iteratives([1,1])

    my_rust_model = py_nrf.UserModel(myModel)
    py_nrf.solve_finite_diff(path, my_rust_model)

    print('Inputs: {}'.format(myModel.inputs))
    print('Left: {}'.format(myModel.left))
    print('Right: {}'.format(myModel.right))
