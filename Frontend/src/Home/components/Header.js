import Typography from "@mui/material/Typography";
import { Button } from "@mui/material";
import { styled } from "@mui/system";
import logo from "../../assets/roog logo x20.png";

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
      <Typography variant="h3" marginTop={-3}>
        Roog Market
      </Typography>
      <hr />
      <Typography variant="h7" marginTop={-3}>
        <b>
          50k (5% of supply) ROOG to start the pool off, up to 8% daily return,
          12% referral.
        </b>
      </Typography>
      <br />
      <br />
      <Button
        variant="contained"
        className="custom-button"
        href="https://jup.ag/swap/USDC-ROOG_8N3ZkCwRe36Cj1PqXaMw2h92yzSy18L1z6sptQMiQGrr"
      >
        Buy $ROOG
      </Button>
    </Wrapper>
  );
}
