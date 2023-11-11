<script lang="ts">
  import * as d3 from 'd3';

  let width = 500;
  let height = 400;
  export let fill: string;
  export let label: string;

  export let data;

  $: root = d3.treemap().size([width, height]).padding(1).round(true)(
    d3
      .hierarchy({
        children: Object.entries(data).map(([key, value]) => ({ key, value }))
      })
      .sum((d) => d.value) // Sum the 'value' property for sizing
  );
</script>

<div>
  <p>{label}</p>
  <svg viewBox="0 0 {width} {height}">
    {#each root.leaves() as leaf}
      <rect
        x={leaf.x0}
        y={leaf.y0}
        width={leaf.x1 - leaf.x0}
        height={leaf.y1 - leaf.y0}
        stroke="black"
        {fill}
      />
      <text
        x={leaf.x0 + 5}
        y={leaf.y0 + 15}
        fill="black"
        font-size="0.8rem"
        font-family="sans-serif"
        text-anchor="start"
      >
        {leaf.data.key}
      </text>
      <text
        x={leaf.x1 - 5}
        y={leaf.y1 - 5}
        fill="black"
        font-size="0.8rem"
        font-family="sans-serif"
        text-anchor="end"
      >
        {leaf.data.value}€ <!-- Access the value property of the leaf node -->
      </text>
    {/each}
  </svg>
</div>

<style lang="scss">
  div {
    min-width: 450px;
    width: 100%;
    display: flex;
    flex-direction: column;
    justify-content: space-between;
    align-items: center;
  }

  svg {
    width: 100%;
    height: 100%;
  }
</style>
