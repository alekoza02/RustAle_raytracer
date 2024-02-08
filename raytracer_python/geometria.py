import numpy as np

class Sfera:

    def __init__(self, origine, radius) -> None:
        self.origine = origine
        self.radius = radius

    def collisione_oggetto(self, raggio):
        
        oc = raggio.origine - self.origine
        a = np.dot(raggio.dir_iterazione, raggio.dir_iterazione)
        b = 2 * np.dot(oc, raggio.dir_iterazione)
        c = np.dot(oc, oc) - self.radius ** 2
        
        discriminante = b ** 2 - 4 * a * c

        if discriminante >= 0:
            return (- b - np.sqrt(discriminante)) / (2*a)
        else:
            return -1

class Scena:

    def __init__(self) -> None:
        self.oggetti = []
        self.luci = []

    def template(self):
        self.oggetti = [
            Sfera(np.array([0,0,0]), 10),
            Sfera(np.array([0,-10010,0]), 10000),
            Sfera(np.array([3,3,15]), 2),
            Sfera(np.array([-8,-8,-20]), 10),
        ]