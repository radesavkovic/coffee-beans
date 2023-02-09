import Typography from "@mui/material/Typography";
import { styled } from "@mui/system";
import logo from "../../assets/FullLogo.png";

const Wrapper = styled("div")(({ theme }) => ({
  textAlign: "center",
  paddingBottom: 24,
  [theme.breakpoints.down("md")]: {
    h5: {
      fontSize: 20,
      margin: 0,
    },
  },
}));

export default function Header() {
  return (
    <Wrapper>
      <img src={logo} alt="" width={"70%"} />
      <hr />
      <Typography variant="h7" marginTop={-3}>
        <b>The SOL reward pool with the richest daily return and lowest dev fee</b>
      </Typography>
    </Wrapper>
  );
}
