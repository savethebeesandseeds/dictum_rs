## Personal notes:
Constitución de Colombia

Codigo CIVIL DE COLOMBIA
Codigo CONTENCIOSO ADMINISTRATIVO
Codigo DE BUEN GOBIERNO
Codigo CONTENCIOSO ADMINISTRATIVO
Codigo DE COMERCIO
Codigo DE CONSTRUCCIÓN DEL DISTRITO CAPITAL DE BOGOTA
Codigo DE CONSTRUCCION SISMO-RESISTENTE
Codigo DE DERECJO CANONICO
Codigo DE EDUCACIÓN
Codigo DE PROCEDIMIENTO CIVIL
Codigo DE PROCEDIMIENTO PENAL
Codigo DE REGIMEN DEPARTAMENTAL
Codigo DE REGIMEN POLITICO Y MINICIPAL
Codigo DEL MENOR
Codigo DE LA INFANCIA Y LA ADOLENSENCIA
Codigo DE MINAS
Codigo NACIONAL DE POLICIA
Codigo DE POLOCIA DE BOGOTA
Codigo DISCIPLINARIO UNICO
Codigo ELECTORAL
Codigo GENERAL DEL PROCESO
Codigo NACIONAL DE RECURSOS NATURALES
Codigo NACIONAL DE TRANSITO TERRESTRE
Codigo PENAL COLOMBIANO
Codigo PENAL MILITAR
Codigo PENITENCIARIOS Y CARCELARIO
Codigo PROCESAL DEL TRABAJO DEL SEGURO SOCIAL
Codigo SUSTANTIVO DEL TRABAJO Y DEL SEGURO SOCIAL
### util websites
https://colombia.justia.com/
### util javascripts 
Filling the law database requires web automation,
```document.querySelector("[id=maincontent]").querySelectorAll("[name]")```

```Array.prototype.slice.call(document.querySelector("[id=maincontent]").querySelectorAll("[name]")).forEach(e=>console.log(e.parentNode.innerHTML))```

```Array.prototype.slice.call(document.querySelectorAll("#main_content > ul:nth-child(1) > li")).forEach(e=>console.log(e.childNodes[1].data))```