class Model:
    def __init__(self):
        self._len_problem = 2 # dummy integer value
        self.right = [0, 0]

    def evaluate(self):
        """
        Your computations, this is a dummy example
        """
        self.left = [elt**2 -2 for elt in self.inputs]

    def len_problem(self):
        """
        Return an integer representing the size of the problem
        """
        return self._len_problem

    def set_iteratives(self, inputs):
        """
        Inputs is expected to be a list of floats
        """
        self.inputs = inputs

    def get_iteratives(self):
        """
        Return a list of floats
        """
        return self.inputs

    def get_residuals(self):
        """
        Return a tuple of two lists of floats
        """
        return (self.left, self.right)
