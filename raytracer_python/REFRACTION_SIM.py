import numpy as np

ir_vetro = 1.333
ir_aria = 1

'''

Legge di Snell:

sin(incidenza) / sin(rifratto) = n2 / n1

'''

for i in [i/10000 for i in range(100000)]:

    print("\n")

    # test angolo di incidenza
    direzione = np.array([i, 1., 0.])
    direzione /= np.linalg.norm(direzione)

    normale = np.array([0.,1.,0.])
    normale /= np.linalg.norm(normale)

    faccia_frontale = True if np.dot(direzione, normale) < 0 else False

    normale = - normale

    rapporto_indici = ir_aria / ir_vetro if faccia_frontale else ir_vetro / ir_aria

    theta_incidente = np.arccos(np.dot(-direzione, normale))
    print(np.rad2deg(theta_incidente))

    sin_theta_incidente = np.sin(theta_incidente)

    sin_theta_rifratto = rapporto_indici * np.sin(theta_incidente)

    print("---------")
    print(f"{sin_theta_rifratto}")
    print("---------")

    if sin_theta_rifratto > 1:
        print("RAGGIUNTO ANGOLO LIMITE!")
        break

    cos_theta_rifratto = np.sqrt(1 - sin_theta_rifratto**2)

    d1_perp = normale * sin_theta_incidente + direzione
    d1_perp /= np.linalg.norm(d1_perp)

    d2_perp = d1_perp * sin_theta_rifratto
    d2_para = - normale * cos_theta_rifratto

    direzione2 = d2_perp + d2_para
    direzione2 /= np.linalg.norm(direzione2)

    print(faccia_frontale, direzione2, np.linalg.norm(direzione2))