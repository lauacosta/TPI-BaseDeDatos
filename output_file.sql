SELECT *
INTO OUTFILE '/home/lautaro/personal/code/TPI_BD/carga_datos/profesores.csv'
FIELDS TERMINATED BY ',' OPTIONALLY ENCLOSED BY '"'
LINES TERMINATED BY '\n'
FROM Profesores;
