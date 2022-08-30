import * as React from "react";
import {Box, FormControl, IconButton, InputLabel, MenuItem, Select, Typography} from "@mui/material";
import {EmojiEmotions, Shuffle, ThumbUpAlt} from "@mui/icons-material";
import Carousel from "react-material-ui-carousel";

const input_team = ["Name1", "Name2", "Name3", "Name4", "Name5", "Name6", "Name7"];

function App() {
  const [list, setList] = React.useState([]);
  const [team, update] = React.useState(input_team);

  const handleChange = (event) => {
    setList([...list, event.target.value]);
    update(team.filter((item) => item !== event.target.value));
  };

  function randomize() {
    setList(list
      .map(value => ({ value, sort: Math.random() }))
      .sort((a, b) => a.sort - b.sort)
      .map(({ value }) => value))
  }

  function Item(props) {
    return (<Typography align="center" sx={{ marginTop: 2, marginBottom: 2 }}>{props.name}</Typography>);
  }

  return (
    <div>
      <Box sx={{ marginLeft: 3, width: 500 }}>
      <Box sx={{ height: 30 }}></Box>
        <Typography variant="h5">popsicle sticks for standup</Typography>
        <EmojiEmotions sx={{ color:"#fbd043" }} fontSize="large"/>
        <ThumbUpAlt sx={{ color:"#fbd043" }} />
        <Box sx={{ height: 30 }}></Box>
        <FormControl variant="standard" sx={{ minWidth: 200 }}>
          <InputLabel id="attendance-label">Select People</InputLabel>
          <Select labelId="attendance-label" id="attendance" onChange={handleChange}>
            {team.map((name) => ( <MenuItem key={name} value={name}>{name}</MenuItem> ))}
          </Select>
        </FormControl>
        <IconButton sx={{ marginTop: 2, marginBottom: 2, marginLeft: 2 }} onClick={() => {randomize()}}>
          <Shuffle />
        </IconButton>
        <Box sx={{ height: 20 }}></Box>
        <Carousel autoPlay={false} indicators={false} sx={{ backgroundColor: "#ddcbb8", borderRadius: "30px", height: "60px" }}> 
          {list.map((name) => (<Item key={name} name={name} />))}
        </Carousel>
      </Box>
    </div>
  );
}

export default App;

