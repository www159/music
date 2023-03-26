import { Box, Grid, Paper, Typography } from "@mui/material";

//SECTION props
interface RecommandListProps {
  label: string
}
//~SECTION
export const RecommandList = (props: RecommandListProps) => {
  const { label } = props;
  return (
    <Paper elevation={3}>
      <Typography>
        label is {label}
      </Typography>
      <Grid
        container
        columns={3}
      >
        <Grid item xs={1}>
          <Typography textAlign={"center"}>
            list1
          </Typography>
        </Grid>
        <Grid item xs={1}>
          <Typography>
            list2
          </Typography>
        </Grid>
        <Grid item xs={1}>
          <Typography>
            list3
          </Typography>
        </Grid>
        <Grid item>
          <Typography>
            list4
          </Typography>
        </Grid>
      </Grid>
    </Paper>
  );
};