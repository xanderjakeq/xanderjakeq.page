<script>
	let styles = {
		'v': '#f4ed2a',
		'note-color': '#FF5555',
		'bg': '#AAAAAA',
	};

	
	$: cssVarStyles = Object.entries(styles)
		.map(([key, value]) => `--${key}:${value}`)
		.join(';')
</script>

<style>
	.container {
		height: 100vh;
		display: flex;
		flex-direction: column;
		justify-content: center;
        align-items: center;

        background-image: url("../images/briight.jpg");
        opacity: .8;
        background-size: cover;
        background-position: center;
	}

    #heading_container {
        width: fit-content;
		margin: .5em 0;
    }

    #sub_container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        width: fit-content;
        align-items: center;
        padding: 1em;

        background-color: rgba(250, 250, 250, 0.5);
        border-radius: 5px;
    }
    

    nav {
        margin-top: 1em;
    }

    h1, h2 {
        opacity: 1;
    }

    nav > a {
        font-family: "Poppins";

        font-weight: 700;
        margin: 0 .5em;
        text-decoration: none;
        opacity: .8;

        transition: all .2s;
    }

    nav > a:hover {
        text-decoration: underline;
        opacity: 1;
    }

    .letter_spacing_1 {
        letter-spacing: 2vw;
        margin-right: -2vw;
    }

    .letter_spacing_2 {
        letter-spacing: .4em;
        margin-right: -.5em;
    }

	h1 {
        font-family: "Josefin Slab";
		font-size: 2.8em;
	/*	text-transform: uppercase; */
		font-weight: 700;
        text-align: center;
	}
    h2 {
        font-family: "Poppins";
        font-weight: semi-bold;
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
			font-size: 4em;
		}
	}
</style>

<svelte:head>
	<title>xanderjakeq | home</title>
</svelte:head>

<div class="container">
	<div id="sub_container">
		<div id="heading_container">
			<h1 class = "letter_spacing_1">xanderjakeq</h1>
			<h2 class = "letter_spacing_2">Xander Jake de los Santos</h2>
		</div>
		<nav>
			<a rel=external target="_blank" href="https://blog.xanderjakeq.page/">blog</a>
			<a rel=external target="_blank" href="https://xanderjakeq.artstation.com/">art</a>
            <a rel=external target="_blank" href="./sketches">sketches</a>
			<a rel=external target="_blank" href="https://github.com/xanderjakeq">github</a>
		</nav>
	</div>
</div>
