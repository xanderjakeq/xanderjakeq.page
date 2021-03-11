<svelte:head>
	<title>Sketches</title>
</svelte:head>

<script>
    import { onMount } from "svelte";
    import InfiniteScroll from "svelte-infinite-scroll";

    let sketches = [];
    let col = [[],[],[]]

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

        let newCol = [[],[],[]]

        sketches.forEach((sketch, i) => {
            newCol[i%newCol.length].push(sketch)
        }) 

        col = [...newCol]

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

    #header-container {
        margin: max(50px, 5%);
        
    }

    h1 {
        font-family: "Josefin Slab";
		font-size: 2.8em;
		font-weight: 700;
        text-align: center;

        margin: 30px 0;
	}

    #main-container {
        display: flex;
        justify-content: center;
        margin: 0 10px;
    }
    
    #grid-container {
        display: grid;
        grid-column-gap: var(--column-gutter);
        grid-template-columns: repeat(var(--columns),minmax(0,1fr));
        align-items: start;
    }

    .column {
        display: grid;
        row-gap: var(--row-gutter);
        grid-template-columns: minmax(0,1fr);
    }

    .img-container{
        max-width: 300px;
        border-radius: 10px;
        overflow: hidden;
    }

    img {
        display: block;
        width: 100%;
        height: auto;
    }

    p {
        font-family: "Poppins";
        text-align: center;
    }

    @media only screen
        and (max-width: 375px){
            #grid-container {
                grid-column-gap: 10px;
                grid-template-columns: repeat(1,minmax(0,1fr));
            }
    }
</style>

<div id="header-container">
    <h1>Sketches</h1>
    <p>recent sketches i post on discord</p>
</div>

<div id="main-container">
    <div id="grid-container" style="--column-gutter:24px;--columns:3">
        <div class="column" style="--row-gutter:24px">
            {#each col[0] as sketch}
                <div class="img-container">
                    <img alt="sketch" src="{sketch.url}">
                </div>
            {/each}
        </div>
        <div class="column" style="--row-gutter:24px">
            {#each col[1] as sketch}
                <div class="img-container">
                    <img alt="sketch" src="{sketch.url}">
                </div>
            {/each}
        </div>
        <div class="column" style="--row-gutter:24px">
            {#each col[2] as sketch}
                <div class="img-container">
                    <img alt="sketch" src="{sketch.url}">
                </div>
            {/each}
        </div>
    
    </div>
    <InfiniteScroll {hasMore} window={true} on:loadMore={handleMore}/>
</div>
