export function disqus_reset(slug) {
    console.log(slug)
    var reset = function () {
        console.log(DISQUS)
        DISQUS.reset({
            reload: true,
            config: function () {
                this.page.url = `https://blinfoldking.dev/post/${slug}`;
                this.page.identifier = `/post/${slug}`;
            }
        });
    };

    try {
        if (slug && slug !== "") {
            setTimeout(reset, 2000)
        }
    } catch (err) {
        console.log(err)
    }
}