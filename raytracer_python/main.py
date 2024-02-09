import numpy as np
from utils import Image
from camera import Camera
from algoritmi import Collisioni
from geometria import Scena

W, H = 50,50
SAMPLES = 100
BOUNCES = 4

post = Image(W,H)
#camera = Camera(np.array([-30, -30, 40]), np.array([0.49960239, 0.49999984, -0.70738787]), np.array([-0.49999984, -0.50039761, -0.70682558]), np.array([0.70738787, -0.70682558, 0.]), np.pi/6)
camera = Camera(np.array([0, 0, 30]), np.array([0, 0, -1]), np.array([0, 1, 0]), np.array([1, 0, 0]), np.pi/3)
scena = Scena()
scena.template()
tester = Collisioni()

prova = np.zeros((W,H,3))

for x in range(W):
    for y in range(H):
        for sample in range(SAMPLES):
            
            camera.genera_direzione(x,y,W,H)
            ray_incoming_light = np.array([0.,0.,0.])
            ray_color = np.array([1.,1.,1.])

            for bounce in range(BOUNCES):
                
                camera.origine_iterante = camera.origine

                info_iter = tester.test_collisione(camera, scena.oggetti)
                
                if info_iter.colpito:
     
                    camera.origine_iterante = info_iter.punto_colpito
                    
                    camera.dir_iterazione = np.array([np.random.normal(),np.random.normal(),np.random.normal()])
                    camera.dir_iterazione /= np.linalg.norm(camera.dir_iterazione)

                    if np.dot(camera.dir_iterazione, info_iter.norma_colpito) <= 0:
                        camera.dir_iterazione = - camera.dir_iterazione
                                    
                    luce_emessa = scena.oggetti[info_iter.indice_sfera].materiale.colore_emi * scena.oggetti[info_iter.indice_sfera].materiale.forza_emi
                    ray_incoming_light = ray_incoming_light + luce_emessa * ray_color
                    ray_color = ray_color * scena.oggetti[info_iter.indice_sfera].materiale.colore
                
                else:
                    break
            
            prova[x,y,:] += np.sqrt(ray_incoming_light) * 255

                
prova /= SAMPLES
prova = np.clip(prova, 0, 255)
post.salva(prova)