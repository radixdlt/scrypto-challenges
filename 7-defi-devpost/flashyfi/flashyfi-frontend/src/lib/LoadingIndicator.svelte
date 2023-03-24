<script lang="ts">
    import CircularProgress from '@smui/circular-progress';
    import Paper, {Content, Title} from "@smui/paper";

    export let text: string
    export let size: string
</script>

<Paper>
    <Title>{text}</Title>
    <Content>
        <div style="display: flex; justify-content: center;">
            <CircularProgress style="height: {size}; width: {size};" indeterminate fourColor/>
        </div>
    </Content>
</Paper>



