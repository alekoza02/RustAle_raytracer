import numpy as np
from utils import Image

W, H = 54, 54
post = Image(W,H)

prova = np.zeros((W,H,3))

for x in range(W):
    for y in range(H):
        prova[x,y,:] = [255 * x / W, 255 * y / H, 255]

post.salva(prova)