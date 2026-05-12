<script lang="ts">
    import { onDestroy, onMount } from "svelte";
    import {
        forceCenter,
        forceCollide,
        forceLink,
        forceManyBody,
        forceSimulation,
        type SimulationLinkDatum,
        type SimulationNodeDatum,
    } from "d3-force";
    import { select } from "d3-selection";
    import { drag } from "d3-drag";
    import { zoom, type D3ZoomEvent } from "d3-zoom";

    import type { ObjectConnectionRecord } from "$lib/connections";
    import {
        displayTitleForObject,
        type ObjectRecord,
    } from "$lib/objects";
    import Icon from "$lib/components/ui/Icon.svelte";

    type GraphNode = ObjectRecord & SimulationNodeDatum;
    type GraphLink = ObjectConnectionRecord &
        SimulationLinkDatum<GraphNode> & {
            source: GraphNode;
            target: GraphNode;
        };
    type GraphSimulation = ReturnType<typeof forceSimulation<GraphNode>>;
    type DragStartEvent = { active: number };
    type DragMoveEvent = DragStartEvent & { x: number; y: number };

    type Props = {
        nodes: ObjectRecord[];
        edges: ObjectConnectionRecord[];
        currentObjectId: string | null;
        onSelectNode: (id: string) => void;
    };

    let { nodes, edges, currentObjectId, onSelectNode }: Props = $props();

    let container = $state<HTMLDivElement | null>(null);
    let svgElement = $state<SVGSVGElement | null>(null);
    let zoomTarget = $state<SVGGElement | null>(null);
    let width = $state(0);
    let height = $state(0);
    let positionedNodes = $state<GraphNode[]>([]);
    let positionedLinks = $state<GraphLink[]>([]);
    let hoveredId = $state<string | null>(null);
    let zoomTransform = $state("translate(0,0) scale(1)");

    let simulation: GraphSimulation | null = null;
    const memoizedPositions = new Map<
        string,
        { x: number; y: number; fx?: number | null; fy?: number | null }
    >();

    const highlightedNodeIds = $derived(neighborSet(hoveredId, edges));
    const highlightedEdgeIds = $derived(incidentEdgeSet(hoveredId, edges));

    onMount(() => {
        const observer = new ResizeObserver(([entry]) => {
            width = entry.contentRect.width;
            height = entry.contentRect.height;
        });

        observer.observe(container!);

        return () => observer.disconnect();
    });

    onDestroy(() => {
        simulation?.stop();
        simulation = null;
    });

    $effect(() => {
        if (!svgElement || !zoomTarget) return;

        const zoomBehavior = zoom<SVGSVGElement, unknown>()
            .scaleExtent([0.2, 5])
            .on(
                "zoom",
                (event: D3ZoomEvent<SVGSVGElement, unknown>) => {
                    zoomTransform = event.transform.toString();
                },
            );

        select(svgElement).call(zoomBehavior);

        return () => {
            select(svgElement).on(".zoom", null);
        };
    });

    $effect(() => {
        simulation?.stop();
        simulation = null;

        if (nodes.length === 0 || width <= 0 || height <= 0) {
            positionedNodes = [];
            positionedLinks = [];
            return;
        }

        const graphNodes: GraphNode[] = nodes.map((node, index) => {
            const previous = memoizedPositions.get(node.id);
            return {
                ...node,
                x: previous?.x ?? width / 2 + Math.cos(index) * 40,
                y: previous?.y ?? height / 2 + Math.sin(index) * 40,
                fx: previous?.fx ?? undefined,
                fy: previous?.fy ?? undefined,
            };
        });

        const nodesById = new Map(graphNodes.map((node) => [node.id, node]));

        const graphLinks: GraphLink[] = edges.flatMap((edge) => {
            const source = nodesById.get(edge.sourceObjectId);
            const target = nodesById.get(edge.targetObjectId);
            if (!source || !target) return [];
            return [{ ...edge, source, target }];
        });

        const nextSimulation = forceSimulation<GraphNode>(graphNodes)
            .force("charge", forceManyBody<GraphNode>().strength(-200))
            .force(
                "link",
                forceLink<GraphNode, GraphLink>(graphLinks).distance(70),
            )
            .force("center", forceCenter(width / 2, height / 2))
            .force("collide", forceCollide<GraphNode>(18))
            .on("tick", () => {
                positionedNodes = [...graphNodes];
                positionedLinks = [...graphLinks];
            })
            .on("end", () => {
                for (const node of graphNodes) {
                    memoizedPositions.set(node.id, {
                        x: node.x ?? 0,
                        y: node.y ?? 0,
                        fx: node.fx,
                        fy: node.fy,
                    });
                }
            });

        positionedNodes = [...graphNodes];
        positionedLinks = [...graphLinks];
        simulation = nextSimulation;

        return () => {
            nextSimulation.stop();
        };
    });

    function dragNode(element: SVGGElement, node: GraphNode) {
        const behavior = drag<SVGGElement, GraphNode>()
            .on("start", (event: DragStartEvent, draggedNode: GraphNode) => {
                if (!event.active) {
                    simulation?.alphaTarget(0.3).restart();
                }
                draggedNode.fx = draggedNode.x;
                draggedNode.fy = draggedNode.y;
            })
            .on("drag", (event: DragMoveEvent, draggedNode: GraphNode) => {
                draggedNode.fx = event.x;
                draggedNode.fy = event.y;
                positionedNodes = [...positionedNodes];
            })
            .on("end", (event: DragStartEvent, draggedNode: GraphNode) => {
                if (!event.active) {
                    simulation?.alphaTarget(0);
                }
                draggedNode.fx = draggedNode.x;
                draggedNode.fy = draggedNode.y;
                memoizedPositions.set(draggedNode.id, {
                    x: draggedNode.x ?? 0,
                    y: draggedNode.y ?? 0,
                    fx: draggedNode.fx,
                    fy: draggedNode.fy,
                });
            });

        function apply(nextNode: GraphNode) {
            select<SVGGElement, GraphNode>(element).datum(nextNode).call(behavior);
        }

        apply(node);

        return {
            update(nextNode: GraphNode) {
                apply(nextNode);
            },
            destroy() {
                select(element).on(".drag", null);
            },
        };
    }

    function nodeRadius(node: GraphNode): number {
        return node.id === currentObjectId ? 8 : 6;
    }

    function handleNodeKeydown(event: KeyboardEvent, id: string) {
        if (event.key !== "Enter" && event.key !== " ") return;
        event.preventDefault();
        onSelectNode(id);
    }

    function neighborSet(
        activeId: string | null,
        graphEdges: ObjectConnectionRecord[],
    ): Set<string> {
        const ids = new Set<string>();
        if (!activeId) return ids;

        ids.add(activeId);
        for (const edge of graphEdges) {
            if (edge.sourceObjectId === activeId) ids.add(edge.targetObjectId);
            if (edge.targetObjectId === activeId) ids.add(edge.sourceObjectId);
        }
        return ids;
    }

    function incidentEdgeSet(
        activeId: string | null,
        graphEdges: ObjectConnectionRecord[],
    ): Set<string> {
        const ids = new Set<string>();
        if (!activeId) return ids;

        for (const edge of graphEdges) {
            if (
                edge.sourceObjectId === activeId ||
                edge.targetObjectId === activeId
            ) {
                ids.add(edge.id);
            }
        }
        return ids;
    }
</script>

<div class="graph" bind:this={container}>
    {#if nodes.length === 0}
        <div class="placeholder">
            <Icon name="graph" size={36} />
            <h3>No graph data yet</h3>
            <p>Create notes and connections to build a global map.</p>
        </div>
    {:else}
        <svg
            bind:this={svgElement}
            class="graph-svg"
            {width}
            {height}
            viewBox={`0 0 ${width} ${height}`}
            role="img"
            aria-label="Object graph"
        >
            <defs>
                <marker
                    id="arrow"
                    viewBox="0 0 10 10"
                    refX="9"
                    refY="5"
                    markerWidth="5"
                    markerHeight="5"
                    orient="auto-start-reverse"
                    markerUnits="strokeWidth"
                >
                    <path d="M 0 0 L 10 5 L 0 10 z" fill="context-stroke"></path>
                </marker>
            </defs>

            <g
                bind:this={zoomTarget}
                class="zoom-target"
                transform={zoomTransform}
            >
                <g class="edges">
                    {#each positionedLinks as edge (edge.id)}
                        <line
                            class="edge"
                            class:highlighted={highlightedEdgeIds.has(edge.id)}
                            class:muted={Boolean(
                                hoveredId && !highlightedEdgeIds.has(edge.id),
                            )}
                            x1={edge.source.x ?? 0}
                            y1={edge.source.y ?? 0}
                            x2={edge.target.x ?? 0}
                            y2={edge.target.y ?? 0}
                            marker-end="url(#arrow)"
                        ></line>
                    {/each}
                </g>

                <g class="nodes">
                    {#each positionedNodes as node (node.id)}
                        {@const title = displayTitleForObject(node)}
                        <g
                            class="node"
                            class:current={node.id === currentObjectId}
                            class:highlighted={highlightedNodeIds.has(node.id)}
                            class:muted={Boolean(
                                hoveredId && !highlightedNodeIds.has(node.id),
                            )}
                            role="button"
                            tabindex="0"
                            aria-label={`Open ${title}`}
                            transform={`translate(${node.x ?? 0}, ${node.y ?? 0})`}
                            use:dragNode={node}
                            onpointerenter={() => (hoveredId = node.id)}
                            onpointerleave={() => (hoveredId = null)}
                            onclick={() => onSelectNode(node.id)}
                            onkeydown={(event) =>
                                handleNodeKeydown(event, node.id)}
                        >
                            <circle r={nodeRadius(node)}></circle>
                            <text x={nodeRadius(node) + 7} y="4">{title}</text>
                        </g>
                    {/each}
                </g>
            </g>
        </svg>
    {/if}
</div>

<style>
    .graph {
        position: absolute;
        inset: 0;
        min-height: 0;
        overflow: hidden;
    }

    .graph-svg {
        display: block;
        width: 100%;
        height: 100%;
        cursor: grab;
        touch-action: none;
    }

    .graph-svg:active {
        cursor: grabbing;
    }

    .zoom-target {
        transform-origin: 0 0;
    }

    .edge {
        stroke: var(--line-3, var(--line-2));
        stroke-width: 1.2;
        opacity: 0.68;
        transition:
            opacity 0.12s,
            stroke 0.12s,
            stroke-width 0.12s;
    }

    .edge.highlighted {
        stroke: var(--accent);
        stroke-width: 1.8;
        opacity: 0.95;
    }

    .edge.muted {
        opacity: 0.14;
    }

    .node {
        cursor: pointer;
        outline: none;
    }

    .node circle {
        fill: var(--bg-elev);
        stroke: var(--fg-3);
        stroke-width: 1.5;
        transition:
            opacity 0.12s,
            fill 0.12s,
            stroke 0.12s,
            stroke-width 0.12s;
    }

    .node text {
        fill: var(--fg-2);
        font-size: 11px;
        font-weight: 500;
        paint-order: stroke;
        pointer-events: none;
        stroke: rgba(253, 252, 250, 0.84);
        stroke-linejoin: round;
        stroke-width: 4px;
        user-select: none;
    }

    .node.current circle {
        fill: var(--accent);
        stroke: var(--accent);
    }

    .node.current text {
        fill: var(--fg-1);
        font-weight: 600;
    }

    .node.highlighted circle,
    .node:focus-visible circle {
        stroke: var(--accent);
        stroke-width: 2;
    }

    .node.muted {
        opacity: 0.24;
    }

    .placeholder {
        position: absolute;
        inset: 0;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        gap: 8px;
        color: var(--fg-3);
        text-align: center;
        padding: 40px;
    }

    .placeholder h3 {
        margin: 6px 0 0;
        font-size: 16px;
        font-weight: 600;
        color: var(--fg-2);
        letter-spacing: -0.01em;
    }

    .placeholder p {
        margin: 0;
        max-width: 320px;
        font-size: 13px;
    }
</style>
