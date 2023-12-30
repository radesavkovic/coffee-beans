import Grid from "@mui/material/Grid";

import { config } from "../../config";
import solIcon from "../assets/SOLIcon.png";
import tgIcon from "../assets/TGIcon.png";
import twIcon from "../assets/TWIcon.png";
import dcIcon from "../assets/DCIcon.png";

export default function Footer() {
  return (
    <Grid container justifyContent="center" spacing={2} marginTop={4}>
      <Grid item>
        <a href="https://x.com/roogBRC20" target="__blank">
          <img src={twIcon} alt="" width={48} height={48} />
        </a>
      </Grid>
      <Grid item>
        <a href="https://t.me/roogBRC20" target="__blank">
          <img src={tgIcon} alt="" width={48} height={48} />
        </a>
      </Grid>
      <Grid item>
        <a href="https://discord.gg/5S4zSZgqKz" target="__blank">
          <img src={dcIcon} alt="" width={48} height={48} />
        </a>
      </Grid>
    </Grid>
  );
}
