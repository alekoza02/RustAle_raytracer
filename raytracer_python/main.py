import numpy as np
from utils import Image
from camera import Camera
from algoritmi import Collisioni
from geometria import Scena

W, H = 54, 54

post = Image(W,H)
#camera = Camera(np.array([-3, -3, 4]), np.array([0.49960239, 0.49999984, -0.70738787]), np.array([-0.49999984, -0.50039761, -0.70682558]), np.array([0.70738787, -0.70682558, 0.]), np.pi/6)
camera = Camera(np.array([0, 0, 30]), np.array([0, 0, -1]), np.array([0, 1, 0]), np.array([1, 0, 0]), np.pi/3)
scena = Scena()
scena.template()
tester = Collisioni()

prova = np.zeros((W,H,3))

for x in range(W):
    for y in range(H):

        camera.genera_direzione(x,y,W,H)
        info_iter = tester.test_collisione(camera, scena.oggetti)

        if info_iter.colpito:
            prova[x,y,:] = [255, 0, 0]
        else:
            prova[x,y,:] = [abs(camera.dir_iterazione[0]) * 255, abs(camera.dir_iterazione[1]) * 255, abs(camera.dir_iterazione[2]) * 255]

post.salva(prova)