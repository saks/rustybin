$('.json').each(function(i, el) {
    const json = JSON.parse(el.innerText);
    $(el).text(JSON.stringify(json, null, 2));
    hljs.highlightBlock(el);
});
