<div class="container">
  <b>Waiting for Opponent</b>
  <svg
    class="spinner"
    width="100px"
    height="100px"
    viewBox="0 0 66 66"
    xmlns="http://www.w3.org/2000/svg"
  >
    <circle
      class="path"
      fill="none"
      stroke-width="6"
      stroke-linecap="round"
      cx="33"
      cy="33"
      r="30"
    ></circle>
  </svg>
</div>

<style>
  div.container {
    display: flex;
    flex-direction: column;
    padding: 10px;
    opacity: 0.9;
    width: 350px;
    height: 200px;
    justify-content: space-around;
    align-items: center;
    background-color: #1b1b1b;
    border: 2px solid #d9d9d9;
    border-radius: 30px;
  }
  b {
    font-size: large;

    color: #d9d9d9;
  }
  .spinner {
    animation: rotator 1.4s linear infinite;
  }
  @keyframes rotator {
    0% {
      transform: rotate(0deg);
    }
    100% {
      transform: rotate(270deg);
    }
  }
  .path {
    stroke-dasharray: 187;
    stroke-dashoffset: 0;
    transform-origin: center;
    animation:
      dash 1.4s ease-in-out infinite,
      colors 5.6s ease-in-out infinite;
  }
  @keyframes colors {
    0% {
      stroke: #d9d9d9;
    }
    25% {
      stroke: #04bfbf;
    }
    50% {
      stroke: #d9d9d9;
    }
    75% {
      stroke: #e53935;
    }
    100% {
      stroke: #d9d9d9;
    }
  }
  @keyframes dash {
    0% {
      stroke-dashoffset: 187;
    }
    50% {
      stroke-dashoffset: 46.75;
      transform: rotate(135deg);
    }
    100% {
      stroke-dashoffset: 187;
      transform: rotate(450deg);
    }
  }
</style>
