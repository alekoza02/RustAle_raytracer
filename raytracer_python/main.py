import numpy as np
from utils import Image

prova = np.array([0,0,0,255,255,255,255,0,0])

post = Image(3,1)
post.salva(prova)