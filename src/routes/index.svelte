<script>
    //import mojs from '@mojs/core';
    import { onMount } from 'svelte';

    let styles = {
        v: '#f4ed2a',
        'note-color': '#FF5555',
        bg: '#AAAAAA'
    };

    $: cssVarStyles = Object.entries(styles)
        .map(([key, value]) => `--${key}:${value}`)
        .join(';');

    const getOffSet = (el) => {
        var rect = el.getBoundingClientRect(),
            scrollLeft =
                window.pageXOffset || document.documentElement.scrollLeft,
            scrollTop =
                window.pageYOffset || document.documentElement.scrollTop;
        return { y: rect.top + scrollTop, x: rect.left + scrollLeft };
    };

    onMount(async () => {
        const mojs = await import('@mojs/core');
        const cont = document.querySelector('#sub_container');
        const links = document.querySelector('#links');

        const base_left = links.getBoundingClientRect().x;

        const shiftCurve = mojs.easing.path(
            'M0,100 C50,100 50,100 50,50 C50,0 50,0 100,0'
        );
        const scaleCurve = mojs.easing.path(
            'M0,100 C21.3776817,95.8051376 50,77.3262711 50,0 C50,80.1708527 76.6222458,93.9449005 100,100'
        );

        const line = new mojs.Shape({
            shape: 'line',
            top: 0,
            left: 0,
            stroke: '#ddde16',
            strokeWidth: 2,
            //parent: links,
            isShowStart: true,
            radiusY: 0,
            scaleX: { 1: 1, curve: scaleCurve },
            origin: { '0 50%': '100% 50%', easing: shiftCurve },
            isForce3d: true
        });

        document.querySelectorAll('.link').forEach((item) => {
            if (item.children.length > 0) return;
            item.addEventListener('mouseenter', (e) => {
                const item = e.target;
                const { x, y } = getOffSet(item);
                const { width, height } = item.getBoundingClientRect();
                console.log(x, y);
                line.tune({
                    parent: item,
                    y: y + height * 0.55,
                    x: x + width * 0.5,
                    radiusX: width - width * 0.5,
                    strokeWidth: height * 0.1
                }).replay();
            });
        });

        const shiftCurve2 = mojs.easing.path(
            'M0,100 C50,100 50,100 50,50 C50,0 50,0 100,0'
        );
        const scaleCurveBase = mojs.easing.path(
            'M0,100 C21.3776817,95.8051376 50,77.3262711 50,-700 C50,80.1708527 76.6222458,93.9449005 100,100'
        );
        const scaleCurve2 = (p) => {
            return 1 + scaleCurveBase(p);
        };
        const nScaleCurve = (p) => {
            return 1 - scaleCurveBase(p) / 10;
        };

        const circle = new mojs.Shape({
            shape: 'circle',
            parent: cont,
            fill: { '#0f030e': '#0f030e', curve: scaleCurve2 },
            radius: 10,
            top: '10%',
            left: '30%',
            rx: 3,
            x: { [-125]: 125, easing: shiftCurve2 },
            scaleX: { 1: 1, curve: scaleCurve2 },
            scaleY: { 1: 1, curve: nScaleCurve },
            origin: { '0 50%': '100% 50%', easing: shiftCurve2 },

            isYoyo: true,
            delay: 1000,
            duration: 800,
            repeat: 3,
            onComplete: () => circle.replay()
        }).play();
    });
</script>

<svelte:head>
    <title>xanderjakeq | home</title>
</svelte:head>

<div class="container">
    <div id="sub_container">
        <div id="heading_container">
            <h1 class="letter_spacing_1">xanderjakeq</h1>
            <h2 class="letter_spacing_2">Xander Jake de los Santos</h2>
        </div>
        <nav id="links">
                class="link"
                rel="external"
                target="_blank"
                href="https://folio.xanderjakeq.page/">projects</a
            >
            <a
                class="link"
                rel="external"
                target="_blank"
                href="https://blog.xanderjakeq.page/">blog</a
            >
            <!--
            <a
                class="link"
                rel="external"
                target="_blank"
                href="https://xanderjakeq.artstation.com/">art</a
            >
            -->
            <a rel="external" target="_blank" href="https://www.twitch.tv/xanderjakeq">
                twitch
            </a>
                sketches
            </a>
            <!--
            <a rel=external target="_blank" href="./commissions">commission</a>
            -->
            <a
                class="link"
                rel="external"
                target="_blank"
                href="https://github.com/xanderjakeq">github</a
            >
        </nav>
    </div>
</div>

<style>
    .container {
        height: 100vh;
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        background-image: url('../images/briight.jpg');
        opacity: 0.8;
        background-size: cover;
        background-position: center;
    }

    #heading_container {
        width: fit-content;
        margin: 0.5em 0;
    }

    #sub_container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        width: fit-content;
        align-items: center;
        padding: 1em;

        background-color: rgba(250, 250, 250, 0.1);
        /*
        background: linear-gradient(
            121deg,
            rgba(55, 173, 255, 0.1) 0%,
            rgba(135, 26, 124, 0.1) 65%,
            rgba(255, 128, 90, 0.1) 100%
        );
        */
        backdrop-filter: blur(2rem);
        border-radius: 10px;
    }

    h1,
    h2 {
        opacity: 1;
    }

    nav {
        display: block;
        margin-top: 1em;
        position: relative;
    }

    nav > a {
        display: inline-block;
        font-family: 'Poppins';
        font-weight: 600;
        font-size: 20px;

        position: relative;

        margin: 0 0.5em;
        text-decoration: none;
        opacity: 0.8;

        transition: all 0.2s;
    }

    /*
    nav > a:hover {
        text-decoration: underline;
        opacity: 1;
    }
    */

    .letter_spacing_1 {
        letter-spacing: 2vw;
        margin-right: -1vw;
    }

    .letter_spacing_2 {
        letter-spacing: 0.4em;
        margin-right: -0.5em;
    }

    h1 {
        font-family: 'Josefin Slab';
        /*  text-transform: uppercase; */
        font-weight: 700;
        text-align: center;
    }
    h2 {
        font-family: 'Poppins';
        text-align: center;
    }

    figure {
        margin: 0 0 1em 0;
    }

    img {
        width: 100%;
        max-width: 400px;
        margin: 0 0 1em 0;
    }

    p {
        margin: 1em auto;
    }

    @media (min-width: 480px) {
        h1 {
        }
    }
</style>
