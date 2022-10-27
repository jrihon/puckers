### How to generate equidistributed points on the surface of a sphere
## Markus Deserno

import numpy as np
from matplotlib import pyplot as plt
"""
_N is equal to theta * phi, based the intervals.
theta [0 - 180], interval = 10
phi [0 - 350], interval = 10

so we sample 18 * 35 = 630 values over the conformation
try 19 * 36 = 684
try 19 * 37 = 703
try 36 * 21 = 756
"""
##_N = 18 * 35
_Ncount = 0
#_N = 630
r = 0.67
area = 4.0 * np.pi * (r*r)
#area_per_point = 0.1
#_N = round(area / area_per_point)
#print(_N)
ratio_volume = r**2

_N = 630
_N_new = _N * ratio_volume

_a = area / (_N_new)
_d = np.sqrt(_a) 
_Mtheta = round(( np.pi / _d ))
_dtheta = ( np.pi / _Mtheta )
_dphi = (_a/_dtheta)

coord_system = open('spherical_coordinates.csv','w')
# Here it comes:
for m in range(0, _Mtheta):
    _THETA = np.pi * (m + 0.5) / _Mtheta
    _Mphi = round((2.0 *np.pi * np.sin(_THETA) / _dphi ))

    for n in range(0,_Mphi):
        _PHI = (2 * np.pi * n) / _Mphi

        ### create points using equation (1)
        """ LEAVE THIS SHIT IN RADIANS"""
        #_PHI = _PHI * (180/np.pi)
        #_THETA = _THETA * (180/np.pi)
        coord_system.write('%.2f   %.2f   %.2f\n' % (r, _THETA, _PHI))
        _Ncount += 1

coord_system.close()

print(_Ncount)


fig = plt.figure()
ax = fig.add_subplot(111, projection='3d')

# Import arrays

test = np.loadtxt('spherical_coordinates.csv')

rho, theta, phi = test[:,0], test[:,1], test[:,2]
#
X = rho*np.sin(theta)*np.cos(phi)
Y = rho*np.sin(theta)*np.sin(phi)
Z = rho*np.cos(theta)

xyz_system = open('cartesian_coordinates.csv','w')
for coords in range(len(X)):
    xyz_system.write('%.4f   %.4f   %.4f\n' % (X[coords], Y[coords], Z[coords]))
xyz_system.close()

exit()
#import pandas as pd
#df = pd.DataFrame({"X": X, "Y": Y, "Z": Z})
#
#xyz = open('xyz_coordinates.csv', 'w')
#for index, row in df.iterrows():
#    xxx, yyy, zzz = row[0], row[1], row[2]
#    xyz.write('%-.4f    %.4f    %.4f\n' % (xxx, yyy, zzz))
#    df.to_csv('xyz_coordinates.csv', header=True, index=False, float_format='%.4f')
#
#xyz.close()
#df.to_csv('xyz_coordinates.csv', header=False, index=False, float_format='%.4f', sep='\t')

Z = np.reshape(Z, (Z.shape[0],1))
#ax.plot_surface(X, Y, Z, cmap=plt.get_cmap('jet'), alpha=0.5)
ax.scatter(X, Y, Z, cmap=plt.get_cmap('jet'), alpha=0.5)
#

ax.set_zlim(-1,1)
ax.set_xlim(-1,1)
ax.set_ylim(-1,1)
ax.set_xlabel('x_axis')
ax.set_ylabel('y_axis')
ax.set_zlabel('z_axis')

plt.show()
