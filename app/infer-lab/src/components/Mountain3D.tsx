import { useMemo } from "react";
import { Milestone, DailyRecord } from "../models/schema";
import "./Mountain.css";

interface Props {
  milestones: Milestone[];
  history: DailyRecord[];
  currentAltitude: number;
}

const MAX_ALTITUDE = 5000;

// 使用种子生成固定的伪随机数
const seededRandom = (seed: number) => {
  const x = Math.sin(seed) * 10000;
  return x - Math.floor(x);
};

// 预生成固定的星星位置
const STARS = Array.from({ length: 50 }, (_, i) => ({
  left: seededRandom(i * 3 + 1) * 100,
  top: seededRandom(i * 3 + 2) * 40,
  delay: seededRandom(i * 3 + 3) * 3,
}));

export function Mountain3D({ milestones, history: _history, currentAltitude }: Props) {
  const progress = useMemo(() => Math.min(currentAltitude / MAX_ALTITUDE, 1), [currentAltitude]);
  
  // Path points for the climbing route (x: 0-100, y: 0-100, y=100 is bottom)
  // Positioned to be visible even with panel open (more center-right)
  const pathPoints = [
    { x: 38, y: 92 },   // 0m - Base
    { x: 44, y: 76 },   // ~500m - Camp 1
    { x: 40, y: 60 },   // ~1500m - Camp 2
    { x: 48, y: 42 },   // ~3000m - Camp 3
    { x: 44, y: 26 },   // ~4000m
    { x: 50, y: 10 },   // 5000m - Summit
  ];

  // Calculate climber position on path
  const climberPos = useMemo(() => {
    const pathProgress = progress * (pathPoints.length - 1);
    const idx = Math.floor(pathProgress);
    const t = pathProgress - idx;
    
    if (idx >= pathPoints.length - 1) {
      return pathPoints[pathPoints.length - 1];
    }
    
    const p1 = pathPoints[idx];
    const p2 = pathPoints[idx + 1];
    return {
      x: p1.x + (p2.x - p1.x) * t,
      y: p1.y + (p2.y - p1.y) * t,
    };
  }, [progress]);

  // Generate path SVG
  const pathD = pathPoints.map((p, i) => 
    `${i === 0 ? 'M' : 'L'} ${p.x} ${p.y}`
  ).join(' ');

  return (
    <div className="mountain-container">
      {/* Sky gradient */}
      <div className="sky" />
      
      {/* Stars - 使用预生成的固定位置 */}
      <div className="stars">
        {STARS.map((star, i) => (
          <div 
            key={i} 
            className="star"
            style={{
              left: `${star.left}%`,
              top: `${star.top}%`,
              animationDelay: `${star.delay}s`,
            }}
          />
        ))}
      </div>

      {/* Mountain layers - back to front */}
      <svg className="mountain-svg" viewBox="0 0 100 100" preserveAspectRatio="xMidYMax slice">
        {/* Far mountain - darkest */}
        <path 
          className="mountain-layer layer-4"
          d="M -5 100 L 10 60 L 25 70 L 40 45 L 55 55 L 70 40 L 85 50 L 100 35 L 105 100 Z"
        />
        
        {/* Mid-far mountain */}
        <path 
          className="mountain-layer layer-3"
          d="M -5 100 L 5 70 L 20 75 L 35 55 L 50 65 L 65 50 L 80 60 L 95 45 L 105 100 Z"
        />
        
        {/* Mid mountain */}
        <path 
          className="mountain-layer layer-2"
          d="M -5 100 L 15 65 L 30 72 L 45 50 L 60 60 L 75 48 L 90 58 L 105 100 Z"
        />
        
        {/* Main mountain (front) - with snow cap */}
        <path 
          className="mountain-layer layer-1"
          d="M -5 100 L 25 75 L 50 8 L 75 70 L 85 62 L 95 70 L 105 100 Z"
        />
        
        {/* Snow cap */}
        <path 
          className="snow-cap"
          d="M 42 26 L 50 8 L 58 26 L 54 23 L 50 28 L 46 23 Z"
        />

        {/* Climbing path */}
        <path 
          className="climb-path"
          d={pathD}
          fill="none"
        />
        
        {/* Path glow */}
        <path 
          className="climb-path-glow"
          d={pathD}
          fill="none"
        />

        {/* Milestone markers */}
        {milestones.map((m) => {
          const altProgress = m.altitude / MAX_ALTITUDE;
          const pathIdx = Math.floor(altProgress * (pathPoints.length - 1));
          const point = pathPoints[Math.min(pathIdx, pathPoints.length - 1)];
          const isReached = currentAltitude >= m.altitude;
          
          return (
            <g key={m.id} className={`milestone ${isReached ? 'reached' : ''}`}>
              {/* Flag pole */}
              <line 
                x1={point.x + 3} 
                y1={point.y} 
                x2={point.x + 3} 
                y2={point.y - 6}
                className="flag-pole"
              />
              {/* Flag */}
              <polygon 
                points={`${point.x + 3},${point.y - 6} ${point.x + 8},${point.y - 4.5} ${point.x + 3},${point.y - 3}`}
                className="flag"
              />
              {/* Camp dot */}
              <circle 
                cx={point.x} 
                cy={point.y} 
                r="1.5"
                className="camp-dot"
              />
            </g>
          );
        })}

        {/* 火把 - 当前位置标记 */}
        <g className="torch">
          {/* 火焰光晕 */}
          <ellipse 
            cx={climberPos.x} 
            cy={climberPos.y - 8} 
            rx="4" 
            ry="5"
            className="flame-glow"
          />
          {/* 火焰主体 */}
          <path 
            d={`M ${climberPos.x} ${climberPos.y - 12} 
                Q ${climberPos.x - 2} ${climberPos.y - 9} ${climberPos.x - 1.5} ${climberPos.y - 6}
                Q ${climberPos.x} ${climberPos.y - 7} ${climberPos.x + 1.5} ${climberPos.y - 6}
                Q ${climberPos.x + 2} ${climberPos.y - 9} ${climberPos.x} ${climberPos.y - 12}`}
            className="flame-outer"
          />
          {/* 火焰内核 */}
          <path 
            d={`M ${climberPos.x} ${climberPos.y - 11} 
                Q ${climberPos.x - 1} ${climberPos.y - 9} ${climberPos.x - 0.8} ${climberPos.y - 7}
                Q ${climberPos.x} ${climberPos.y - 7.5} ${climberPos.x + 0.8} ${climberPos.y - 7}
                Q ${climberPos.x + 1} ${climberPos.y - 9} ${climberPos.x} ${climberPos.y - 11}`}
            className="flame-inner"
          />
          {/* 火把柄 */}
          <rect 
            x={climberPos.x - 0.8} 
            y={climberPos.y - 6} 
            width="1.6" 
            height="6"
            rx="0.4"
            className="torch-handle"
          />
          {/* 底座点 */}
          <circle 
            cx={climberPos.x} 
            cy={climberPos.y} 
            r="1"
            className="torch-base"
          />
        </g>

        {/* Summit marker */}
        <g className="summit-marker">
          <polygon 
            points="50,5 52,0 54,5"
            className="summit-flag"
          />
          <text x="50" y="-3" className="summit-text">🏆</text>
        </g>
      </svg>

      {/* Ground/fog at bottom */}
      <div className="ground-fog" />
      
      {/* Altitude scale on the right */}
      <div className="altitude-scale">
        {milestones.map((m) => (
          <div 
            key={m.id}
            className={`scale-mark ${currentAltitude >= m.altitude ? 'reached' : ''}`}
            style={{ bottom: `${(m.altitude / MAX_ALTITUDE) * 80 + 10}%` }}
          >
            <span className="scale-line" />
            <span className="scale-label">{m.altitude}m</span>
          </div>
        ))}
      </div>
    </div>
  );
}
