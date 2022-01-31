<script>
    import { onMount } from 'svelte';
    import InfiniteScroll from 'svelte-infinite-scroll';

    let MOBILE_WIDTH = 500;

    let monetized = false;

    let window_width;
    $: mobile = window_width < MOBILE_WIDTH;

    let sketches = [];
    let col = [[], [], []];

    let user = '305874292397178882';
    let before = '';
    let hasMore = true;

    const getSketches = async (user_id, earliest) => {
        const local = 'http://localhost:5000/';
        const prod = 'https://skootches.herokuapp.com/';
        const res = await fetch(
            `${prod}api/sketches?` +
                new URLSearchParams({
                    user_id,
                    earliest
                })
        );
        const data = await res.json();
        before = data.earliest;
        hasMore = sketches.length < 30 && data.attachments.length > 0;

        sketches = [...sketches, ...data.attachments];

        let newCol = [[], [], []];

        sketches.forEach((sketch, i) => {
            newCol[i % newCol.length].push(sketch);
        });

        col = [...newCol];

        if (sketches.length < 20) {
            getSketches(user_id, data.earliest);
        }
    };

    const handleMore = (e) => {
        e.preventDefault();
        getSketches(user, before);
    };

    onMount(async function () {
        if (document.monetization) {
            document.monetization.addEventListener('monetizationstart', () => {
                monetized = true;
            });
        }

        await getSketches(user, before);
    });
</script>

<svelte:head>
    <title>Sketches</title>
    <meta name="monetization" content="$ilp.uphold.com/q8weYzAPqbGh" />
</svelte:head>

<svelte:window bind:innerWidth={window_width} />

<div id="header-container" class="flex-container-centered">
    <h1>Sketches</h1>
    <p>recent sketches i post on discord</p>
</div>

<div id="main-container">
    <div
        id="grid-container"
        style={mobile
            ? '--column-gutter: 10px;--columns:1'
            : '--column-gutter:24px;--columns:3'}
    >
        {#if !mobile}
            <div class="column" style="--row-gutter:24px">
                {#each col[0] as sketch, index}
                    <div
                        class="img-container {index === 0
                            ? monetized
                                ? 'featured'
                                : 'hidden'
                            : null}"
                    >
                        <img alt="sketch" src={sketch.url} />
                    </div>
                {/each}
            </div>
            <div class="column" style="--row-gutter:24px">
                {#each col[1] as sketch, index}
                    <div
                        class="img-container {index === 0
                            ? monetized
                                ? 'featured'
                                : 'hidden'
                            : null}"
                    >
                        <img alt="sketch" src={sketch.url} />
                    </div>
                {/each}
            </div>
            <div class="column" style="--row-gutter:24px">
                {#each col[2] as sketch, index}
                    <div
                        class="img-container {index === 0
                            ? monetized
                                ? 'featured'
                                : 'hidden'
                            : null}"
                    >
                        <img alt="sketch" src={sketch.url} />
                    </div>
                {/each}
            </div>
        {:else}
            <div class="column" style="--row-gutter:24px">
                {#each sketches as sketch, index}
                    <div
                        class="img-container {index < 3
                            ? monetized
                                ? 'featured'
                                : 'hidden'
                            : null}"
                    >
                        <img alt="sketch" src={sketch.url} />
                    </div>
                {/each}
            </div>
        {/if}
    </div>

    <InfiniteScroll {hasMore} window={true} on:loadMore={handleMore} />
</div>

{#if !hasMore}
    <footer class="flex-container-centered">
        <div>
            <p>
                This is web-monetized through <a href="https://coil.com/"
                    >coil</a
                >. If you are subscribed, you will see the 3 most recent
                sketches.
                <br />
                <span class="emoji">🤓</span>
            </p>
            <p>
                <small>
                    &copy; Copyright {new Date().getUTCFullYear()}, xanderjakeq.
                    All rights reserved.
                </small>
            </p>
            <div />
        </div>
    </footer>
{/if}

<style>
    .flex-container-centered {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .hidden {
        display: none;
    }

    .featured {
        box-shadow: 0 0 0 3pt rgba(56, 199, 34, 0.63);
    }

    .emoji {
        font-size: 30px;
    }

    #header-container {
        margin: max(50px, 5%);
    }

    #main-container {
        display: flex;
        justify-content: center;
        margin: 10px;
    }

    #grid-container {
        display: grid;
        grid-column-gap: var(--column-gutter);
        grid-template-columns: repeat(var(--columns), minmax(0, 1fr));
        align-items: start;
    }

    .column {
        display: grid;
        row-gap: var(--row-gutter);
        grid-template-columns: minmax(0, 1fr);
    }

    .img-container {
        max-width: 300px;
        border-radius: 10px;
        overflow: hidden;
    }

    h1 {
        font-family: 'Josefin Slab';
        font-size: 2.8em;
        font-weight: 700;
        text-align: center;

        margin: 30px 0;
    }

    img {
        display: block;
        width: 100%;
        height: auto;
    }

    p {
        font-family: 'Poppins';
        text-align: left;
        word-wrap: wrap;

        line-height: normal;

        max-width: 50ch;
        padding: 5px 0;
    }

    footer {
        min-height: 100px;
        margin: 10px;
    }

    /*

    @media only screen
        and (max-width: 500px){
            #grid-container {
                grid-column-gap: 10px;
                grid-template-columns: repeat(1,minmax(0,1fr));
            }
    }
    */
</style>
