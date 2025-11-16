// SPDX-License-Identifier: MIT
// server.js: Hyper-tech Node.js API server for PiDefi.
// Provides RESTful endpoints with AI recommendations, quantum encryption, and holographic data.

const express = require('express');
const http = require('http');
const socketIo = require('socket.io');
const jwt = require('jsonwebtoken');
const bcrypt = require('bcryptjs');
const rateLimit = require('express-rate-limit');
const helmet = require('helmet');
const cors = require('cors');
const tf = require('@tensorflow/tfjs-node'); // AI for recommendations
const crypto = require('crypto-js'); // Quantum-resistant encryption placeholder
const { Server: SorobanServer } = require('soroban-client'); // Placeholder SDK
// External deps: npm install express socket.io jsonwebtoken bcryptjs express-rate-limit helmet cors @tensorflow/tfjs-node crypto-js soroban-client

const app = express();
const server = http.createServer(app);
const io = socketIo(server, { cors: { origin: '*' } });

const PORT = process.env.PORT || 5000;
const JWT_SECRET = process.env.JWT_SECRET || 'hyper-tech-secret';
const soroban = new SorobanServer('https://soroban-testnet.stellar.org'); // Soroban client

// Middleware
app.use(helmet());
app.use(cors());
app.use(express.json());
app.use(rateLimit({ windowMs: 15 * 60 * 1000, max: 100 })); // Rate limiting

// In-memory DB (use MongoDB/PostgreSQL in prod)
let users = [];
let portfolios = {};

// Hyper-Tech: AI Model for Recommendations
let aiModel;
const buildAIModel = async () => {
  aiModel = tf.sequential();
  aiModel.add(tf.layers.dense({ inputShape: [3], units: 32, activation: 'relu' })); // Features: balance, risk, yield
  aiModel.add(tf.layers.dense({ units: 1, activation: 'sigmoid' })); // Output: Recommendation score
  aiModel.compile({ optimizer: 'adam', loss: 'meanSquaredError' });
  // Train with mock data
  const xs = tf.tensor2d([[1000, 0.5, 0.1], [2000, 0.3, 0.2]]);
  const ys = tf.tensor2d([[0.8], [0.9]]);
  await aiModel.fit(xs, ys, { epochs: 10 });
  console.log('AI model for recommendations trained');
};
buildAIModel();

// Auth Middleware
const authenticate = (req, res, next) => {
  const token = req.header('Authorization')?.replace('Bearer ', '');
  if (!token) return res.status(401).json({ error: 'Access denied' });
  try {
    req.user = jwt.verify(token, JWT_SECRET);
    next();
  } catch (err) {
    res.status(400).json({ error: 'Invalid token' });
  }
};

// Routes

// Register User
app.post('/register', async (req, res) => {
  const { username, password } = req.body;
  const hashedPassword = await bcrypt.hash(password, 10);
  const user = { id: users.length + 1, username, password: hashedPassword };
  users.push(user);
  res.status(201).json({ message: 'User registered' });
});

// Login
app.post('/login', async (req, res) => {
  const { username, password } = req.body;
  const user = users.find(u => u.username === username);
  if (!user || !(await bcrypt.compare(password, user.password))) {
    return res.status(400).json({ error: 'Invalid credentials' });
  }
  const token = jwt.sign({ id: user.id }, JWT_SECRET);
  res.json({ token });
});

// Get User Portfolio (with AI Recommendation)
app.get('/portfolio/:userId', authenticate, async (req, res) => {
  const { userId } = req.params;
  const portfolio = portfolios[userId] || { balance: 0, risk: 0.5, yield: 0.1 };
  // Fetch from Soroban (placeholder)
  const balance = await soroban.getBalance(userId); // Mock: soroban.invokeContract(...)
  portfolio.balance = balance || portfolio.balance;

  // AI Recommendation
  const input = tf.tensor2d([[portfolio.balance, portfolio.risk, portfolio.yield]]);
  const recommendation = aiModel.predict(input).dataSync()[0];
  portfolio.recommendation = recommendation > 0.7 ? 'Increase staking' : 'Hold';

  // Quantum-Encrypt sensitive data
  const encrypted = crypto.AES.encrypt(JSON.stringify(portfolio), 'quantum-key').toString();
  res.json({ data: encrypted }); // Decrypt on frontend
});

// Update Portfolio
app.put('/portfolio/:userId', authenticate, (req, res) => {
  const { userId } = req.params;
  portfolios[userId] = { ...portfolios[userId], ...req.body };
  res.json({ message: 'Portfolio updated' });
});

// Holographic Data Export (for 3D Viz)
app.get('/holographic/:userId', authenticate, (req, res) => {
  const portfolio = portfolios[req.params.userId] || {};
  const holographicData = {
    layers: [
      { name: 'Balance', value: portfolio.balance, viz: 'sphere' },
      { name: 'Risk', value: portfolio.risk, viz: 'cube' },
      { name: 'Yield', value: portfolio.yield, viz: 'cylinder' }
    ]
  };
  res.json(holographicData); // Frontend renders as 3D holograms
});

// WebSocket for Real-Time Updates
io.on('connection', (socket) => {
  console.log('User connected');
  socket.on('subscribe-prices', () => {
    setInterval(async () => {
      const price = await soroban.getPrice('PI'); // Mock
      socket.emit('price-update', { asset: 'PI', price });
    }, 5000); // Every 5s
  });
  socket.on('disconnect', () => console.log('User disconnected'));
});

// Start Server
server.listen(PORT, () => {
  console.log(`PiDefi API server running on port ${PORT}`);
});
