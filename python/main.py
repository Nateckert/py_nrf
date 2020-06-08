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
