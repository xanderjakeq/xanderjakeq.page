<svelte:head>
	<title>Sketches</title>
</svelte:head>

<script>
    import { onMount } from "svelte";
    import InfiniteScroll from "svelte-infinite-scroll";
    let sketches = [];

    let user = "305874292397178882";
    let before = "";
    let hasMore = true;

    const getSketches = async (user_id, earliest) => {
        const local = "http://localhost:5000/";
        const prod = "https://skootches.herokuapp.com/";
        const res = await fetch(`${prod}api/sketches?` + new URLSearchParams({
            user_id,
            earliest
        }));
        const data = await res.json();
        before = data.earliest;
        hasMore = sketches.length < 30 && data.attachments.length > 0 ;

        sketches = [...sketches, ...data.attachments];

        if (sketches.length < 20) {
            getSketches(user_id, data.earliest)
        }
    }

    const handleMore = (e) => {
        e.preventDefault()
        getSketches(user, before)
    }

    onMount(async function() {
        getSketches(user, before)
    });
</script>

<style>
    h1 {
        font-family: "Josefin Slab";
		font-size: 2.8em;
	/*	text-transform: uppercase; */
		font-weight: 700;
        text-align: center;

        margin: 30px 0;
	}

    #mainContainer {
        display: flex;
        justify-content: center;
    }
    
    #classContainer {
        display: grid;
        max-width: 1000px;
        grid-template-columns: repeat(3, 1fr); 
        justify-items: center;
        align-items: center;

    }

    img {
        max-width: 300px;
        max-height: 300px;
        
        overflow: hidden;

        margin: 10px;
    }

    p {
        font-family: "Poppins";
        text-align: center;
    }
</style>

<h1>Sketches</h1>

<p>recent sketches i post on discord</p>
<div id = "mainContainer">
    <div id="classContainer">
    {#each sketches as sketch}
            <img alt="sketch" src="{sketch.url}">
    {/each}
    </div>
    <InfiniteScroll {hasMore} window={true} on:loadMore={handleMore}/>
</div>
