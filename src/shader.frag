#version 100
precision lowp float;
varying vec2 uv;
varying vec2 uv_screen;
varying vec2 center;

uniform vec2 ray;
uniform vec2 resolution;
uniform sampler2D normals;
uniform sampler2D albedo;
uniform sampler2D remap;
uniform sampler2D _ScreenTexture;
uniform float ratio;
uniform float time;
void main() {
    float gradient = length(uv);
    vec2 vector=(uv_screen - center);
    vec2 uv_zoom = vector * gradient + center;
    vec2 n=normalize(center);

    vec4 col = texture2D(_ScreenTexture, uv_screen);
    vec4 alb = texture2D(albedo,uv_screen/ratio);
    vec2 pixs=floor(vec2(resolution.x*uv_screen.x,resolution.y*uv_screen.y));
    vec4 norms=texture2D(normals,uv_screen/ratio); //ints
    if(col.b>0.){
        
        //ints+=.1;
        
        if(norms.b>.0){
            vec3 n=normalize(vec3(.5-norms.r,.5-norms.g,norms.b-.5)); //.5-norms.g
            vec2 v=normalize(vector);
            float c=normalize(vec2(n.x*v.x,n.y*v.x)).x;
            //t = glm::normalize(t - n * glm::dot(n, t));
            //vec2 uv2 = normalize(ray-n*dot(n,ray));//(uv-0.5*uv_screen.xy)/uv_screen.y;
            vec2 new_ray=((uv_screen)-ray);
            float f = dot(vec3(new_ray,1.),n);

        
            if(uv_screen.x>.99){
                
               // gl_FragColor = mix(alb*.6,alb,f);
                gl_FragColor = vec4(alb);
            }else{
                //.2 to .6
                if(f>=.4 && f<.6){
                    if(mod(pixs.x+pixs.y/2.,4.)==0.){
                        f=0.;
                    }else{
                        f=1.;
                    }
                }else if(f>=.3 && f<.4){
                    if(mod(pixs.x+pixs.y/2.,2.)==0.){
                        f=0.;
                    }else{
                        f=1.;
                    }
                }else if (f<.3 && f>=.1){
                    if(mod(pixs.x+pixs.y,2.)==0.){
                        f=0.;
                    }else{
                        f=1.;
                    }
                }else if(f<.1){
                    f=0.;
                
                }else{
                    f=1.;
                }
                /*if(mod(pixs.x/2.+pixs.y/8.,2.)==0.){
                    f=0.;
                }else{
                    f=1.;
                }*/
                //$246 9
                //gb 48 96 130 255
                //63 63 116 255
                float r=63./256.;
                float g=63./256.;
                float b=116./256.;
                //(floor(r*16.)+floor(b*16.)*16.)/128. +
                //(g+16.)/32.
                vec4 c2=texture2D(remap,vec2((floor(alb.r*16.)/256.+floor(alb.b*16.)/16.),.5+alb.g/2.));  //(alb.x/16.+alb.z)/16.
                vec4 c=texture2D(remap, vec2(uv_screen.x,uv_screen.y)); //vec2((alb.x)/256.+(alb.z)/16.
                gl_FragColor = mix(c2,alb,f);
            }
            //gl_FragColor = vec4(col);//vec4(1,0,0,1);//mix(col*.6,col,f);//vec4(f,0.,0.,f);
        }else{
            gl_FragColor = vec4(alb);
        }
        //gl_FragColor=texture2D(remap, vec2(uv_screen.x*2.,uv_screen.y*8.));
    }else{ 
        gl_FragColor = vec4(alb);
    }
    gl_FragColor.r=0.;
    //gl_FragColor = vec4(1.,0.,0.,1.);
    /*if(norms.b==0.){
        gl_FragColor=vec4(1,0,0,1);
    }*/
}
