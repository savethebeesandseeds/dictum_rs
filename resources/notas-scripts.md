## Personal notes:
Filling the law database requires web automation,
### util websites
https://colombia.justia.com/
### util javascripts 
```document.querySelector("[id=maincontent]").querySelectorAll("[name]")```

```Array.prototype.slice.call(document.querySelector("[id=maincontent]").querySelectorAll("[name]")).forEach(e=>console.log(e.parentNode.innerHTML))```

```Array.prototype.slice.call(document.querySelectorAll("#main_content > ul:nth-child(1) > li")).forEach(e=>console.log(e.childNodes[1].data))```