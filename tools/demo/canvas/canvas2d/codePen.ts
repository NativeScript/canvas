export function codePen(canvas){
     /*   var ctx = canvas.getContext("2d"),
        w = canvas.width = window.innerWidth,
        h = canvas.height = window.innerHeight,
        logoParticles = [],
        particleIndex = 0,
        logo = new Image(),
        hue = 0;


    function Particle(x, y){
        this.origX = this.x = x;
        this.origY = this.y = y;
        this.color = "white";
        this.maxLife = this.random(20);
        this.life = 0;
        this.vx = this.random(-1, 1);
        this.vy = this.random(-1, 1);
        this.grav = .04;
        this.index = particleIndex;
        logoParticles[particleIndex] = this;
        particleIndex++;
    }

    Particle.prototype = {

        constructor: Particle,

        draw: function(){
            ctx.fillStyle = this.color;
            ctx.fillRect(this.x, this.y, 2, 2);
            this.update();
        },

        update: function(){
            if(this.life >= this.maxLife){
                logoParticles[this.index].reset();
            }
            this.x += this.vx;
            this.y += this.vy;
            this.vy += this.grav;
            this.life++;
        },

        reset: function(){
            this.x = this.origX;
            this.y = this.origY;
            this.life = 0;
            this.color = "hsl("+hue+", 100%, 50%)";
            this.vx = this.random(-1, 1);
            this.vy = this.random(-1, 1);
        },

        random: function(){
            var
                min = arguments.length == 1 ? 0 : arguments[0],
                max = arguments.length == 1 ? arguments[0] : arguments[1];
            return Math.random() * (max - min) + min;
        }

    };

    logo.src = "data:image/png;base64,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";

    logo.onload = function(){
        var
            posX = (w - this.width)/2,
            posY = (h - this.height)/2;

        ctx.drawImage(this, posX, posY);

        var
            imgData = ctx.getImageData(0, 0, w, h),
            pixels = imgData.data;

        for(var y = 0; y < imgData.height; y+=3) {
            for(var x = 0; x < imgData.width; x+=3) {
                var alpha = pixels[((imgData.width * y) + x) * 4 + 3];
                if(alpha > 0){
                    logoParticles.push(new Particle(x, y));
                }
            }
        }

        setTimeout(function(){
            animate();
        }, 800);

    };

    function animate(){
        ctx.fillStyle = "rgba(0,0,0,.1)";
        ctx.fillRect(0,0,w,h);

        for(var i in logoParticles){
            logoParticles[i].draw();
        }

        hue += 1;
        window.requestAnimationFrame(animate);*/
}
