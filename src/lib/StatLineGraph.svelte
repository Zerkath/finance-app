<script lang="ts">
  import * as d3 from 'd3';

  let width = 500;
  let height = 800;
  export let label: string;

  export let data;

  const getMinMax = (d) => {
    let min = Number.MAX_VALUE;
    let max = Number.MIN_VALUE;
    for (const { key, value } of d) {
      if (value < min) {
        min = value;
      }
      if (value > max) {
        max = value;
      }
    }
    return [min - 15, max + 120];
  };

  let axisBottom;
  let axisLeft;

  $: data, console.log(data);
  $: x = d3.scaleLinear().domain(getMinMax(data)).range([0, width]);
  $: y = d3
    .scaleBand()
    .range([0, height])
    .domain(
      data.map(function (row) {
        return row.key;
      })
    )
    .padding(1);

  $: d3.select(axisBottom)
    .call(d3.axisBottom(x))
    .selectAll('text')
    .style('text-anchor', 'end');

  $: d3.select(axisLeft)
    .call(d3.axisLeft(y))
    .selectAll('path')
    .attr('transform', `translate(${x(0)}, 0)`);

  $: d3.select(axisLeft)
    .call(d3.axisLeft(y))
    .selectAll('line')
    .attr('stroke', 'none');
</script>

<div>
  <p>{label}</p>
  <svg {width} {height}>
    <g transform="translate(80, 0)">
      <g bind:this={axisBottom} transform="translate(0, {height - 20})" />
      <g bind:this={axisLeft} transform="translate(0, 0)" />
      {#each data as row}
        <line
          x1={x(row.value)}
          x2={x(0)}
          y1={y(row.key)}
          y2={y(row.key)}
          stroke={row.value > 0 ? '#a3e635' : '#ea580c'}
          stroke-width="3.2"
        />
        <circle
          cx={x(row.value)}
          cy={y(row.key)}
          r="4"
          fill={row.value > 0 ? '#a3e635' : '#ea580c'}
          stroke="black"
        />
      {/each}
    </g>
  </svg>
</div>
