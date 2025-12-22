import { useEffect, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import Timeline from "@mui/lab/Timeline";
import TimelineItem from "@mui/lab/TimelineItem";
import TimelineSeparator from "@mui/lab/TimelineSeparator";
import TimelineConnector from "@mui/lab/TimelineConnector";
import TimelineContent from "@mui/lab/TimelineContent";
import TimelineDot from "@mui/lab/TimelineDot";
import TimelineOppositeContent from "@mui/lab/TimelineOppositeContent";
import Typography from "@mui/material/Typography";
import Box from "@mui/material/Box";
import FastfoodIcon from "@mui/icons-material/Fastfood";
import LaptopMacIcon from "@mui/icons-material/LaptopMac";
import HotelIcon from "@mui/icons-material/Hotel";
import RepeatIcon from "@mui/icons-material/Repeat";
import "./App.css";

interface TimelineEvent {
  id: string;
  time: string;
  title: string;
  description: string;
  color: "primary" | "secondary" | "success" | "error" | "info" | "warning" | "grey";
  icon: string;
}

const iconMap: { [key: string]: React.ReactElement } = {
  Fastfood: <FastfoodIcon />,
  LaptopMac: <LaptopMacIcon />,
  Hotel: <HotelIcon />,
  Repeat: <RepeatIcon />,
};

function App() {
  const [events, setEvents] = useState<TimelineEvent[]>([]);
  const [loading, setLoading] = useState(true);

  useEffect(() => {
    invoke<TimelineEvent[]>("get_timeline_events")
      .then((data) => {
        setEvents(data);
      })
      .catch((err) => {
        console.error("加载事件失败:", err);
      })
      .finally(() => {
        setLoading(false);
      });
  }, []);

  if (loading) {
    return (
      <Box sx={{ display: "flex", justifyContent: "center", alignItems: "center", minHeight: "100vh" }}>
        <Typography variant="h6" color="text.secondary">
          加载中...
        </Typography>
      </Box>
    );
  }

  return (
    <Box sx={{ padding: 4 }}>
      <Typography variant="h4" gutterBottom align="center">
        Infer-Lab
      </Typography>
      
      <Timeline position="alternate">
        {events.map((event) => (
          <TimelineItem key={event.id}>
            <TimelineOppositeContent
              sx={{ m: "auto 0" }}
              variant="body2"
              color="text.secondary"
            >
              {event.time}
            </TimelineOppositeContent>
            
            <TimelineSeparator>
              <TimelineConnector />
              <TimelineDot color={event.color}>
                {iconMap[event.icon]}
              </TimelineDot>
              <TimelineConnector />
            </TimelineSeparator>
            
            <TimelineContent sx={{ py: "12px", px: 2 }}>
              <Typography variant="h6" component="span">
                {event.title}
              </Typography>
              <Typography>{event.description}</Typography>
            </TimelineContent>
          </TimelineItem>
        ))}
      </Timeline>
    </Box>
  );
}

export default App;
